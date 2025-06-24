use clap::ArgMatches;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::{stderr, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command as ProcessCommand};

mod dirs;
mod term;

struct Command<'a> {
    name: &'a str,
    args: Vec<String>,
}

struct CommandHistory {
    entries: VecDeque<String>,
    max_size: usize,
    history_file: PathBuf,
}

impl CommandHistory {
    fn new(max_size: usize) -> io::Result<Self> {
        let history_file = dirs::app_data_home("smash")?.join("history");

        let mut history = CommandHistory {
            entries: VecDeque::with_capacity(max_size),
            max_size,
            history_file,
        };

        // Load history from file if it exists
        // Don't fail the whole program if history can't be loaded
        if let Err(e) = history.load() {
            eprintln!("Warning: Could not load command history: {}", e);
        }

        Ok(history)
    }

    fn add(&mut self, command: &str) -> io::Result<()> {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        // Don't add duplicate of the most recent command
        if let Some(last) = self.entries.back() {
            if last == trimmed {
                return Ok(());
            }
        }

        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }

        self.entries.push_back(trimmed.to_string());

        // Try to save but don't fail if saving fails
        if let Err(e) = self.save() {
            eprintln!("Warning: Failed to save command history: {}", e);
        }

        Ok(())
    }

    fn load(&mut self) -> io::Result<()> {
        if !self.history_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.history_file)?;
        self.entries = content.lines().map(String::from).collect::<VecDeque<_>>();

        // Ensure we don't exceed max size
        while self.entries.len() > self.max_size {
            self.entries.pop_front();
        }

        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.history_file.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = self
            .entries
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // Attempt to write to file, handle I/O errors gracefully
        match fs::write(&self.history_file, content) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Warning: Could not save command history: {}", e);
                Ok(()) // Carry on even if we can't save history
            }
        }
    }

    fn get(&self, index: usize) -> Option<&String> {
        self.entries.get(index)
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

pub fn run_loop(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut history = CommandHistory::new(1000)?;
    let mut exiting = false;
    let posix_mode = matches.get_flag("posix");

    if posix_mode {
        init_posix()?;
    }

    while !exiting {
        // Print a newline before the prompt (except on the first prompt)
        if !history.entries.is_empty() {
            println!();
        }

        // Don't exit the shell on prompt errors
        if let Err(e) = term::prompt::print_prompt() {
            eprintln!("Error displaying prompt: {}", e);
            // Ensure we still have some kind of prompt
            print!("$ ");
            stdout().flush()?;
        }

        match read_input() {
            Ok(input) => {
                // Add command to history if not empty
                if !input.trim().is_empty() {
                    // Ignore errors when adding to history
                    let _ = history.add(&input);
                }

                // Expand environment variables and tildes with better error handling
                let expanded_string = match shellexpand::full(&input) {
                    Ok(expanded) => expanded.to_string(),
                    Err(e) => {
                        eprintln!("Warning: Error expanding variables: {}", e);
                        input.clone() // Use original input if expansion fails
                    }
                };

                let command = parse_command(&expanded_string);

                match command {
                    Some(Command { name: "exit", .. }) => exiting = true,
                    Some(Command { name: "echo", args }) => {
                        // Handle -n flag (no newline)
                        let mut skip_newline = false;
                        let mut output_args = vec![];

                        for arg in &args {
                            if arg == "-n" && output_args.is_empty() {
                                skip_newline = true;
                            } else {
                                output_args.push(arg);
                            }
                        }

                        print!(
                            "{}",
                            output_args
                                .iter()
                                .map(|s| s.as_str())
                                .collect::<Vec<_>>()
                                .join(" ")
                        );
                        if !skip_newline {
                            println!();
                        }
                        stdout().flush()?;
                    }
                    Some(Command { name: "env", .. }) => {
                        for (key, value) in env::vars() {
                            println!("{}={}", key, value);
                        }
                    }
                    Some(Command { name: "cd", args }) => {
                        let default_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
                        let dir = args.first().map(|d| d.as_str()).unwrap_or(&default_dir);

                        // Handle "cd -" to go to previous directory
                        let target_dir = if dir == "-" {
                            if let Ok(oldpwd) = env::var("OLDPWD") {
                                println!("{}", oldpwd);
                                oldpwd
                            } else {
                                println!("smash: cd: OLDPWD not set");
                                continue;
                            }
                        } else {
                            dir.to_string()
                        };

                        // Save current directory before changing
                        if let Ok(current_dir) = env::current_dir() {
                            env::set_var("OLDPWD", current_dir.to_string_lossy().to_string());
                        }

                        match env::set_current_dir(&target_dir) {
                            Ok(_) => {
                                if let Ok(new_dir) = env::current_dir() {
                                    env::set_var("PWD", new_dir.to_string_lossy().to_string());
                                }
                            }
                            Err(e) => println!("smash: cd: {}: {}", target_dir, e),
                        }
                    }
                    Some(Command { name: "pwd", .. }) => {
                        if let Ok(current_dir) = env::current_dir() {
                            println!("{}", current_dir.display());
                        } else {
                            println!("smash: pwd: Unable to determine current directory");
                        }
                    }
                    Some(Command { name: "clear", .. }) => {
                        print!("\x1B[2J\x1B[1;1H");
                        stdout().flush()?;
                    }
                    Some(Command {
                        name: "history", ..
                    }) => {
                        for (i, cmd) in history.entries.iter().enumerate() {
                            println!("{:5} {}", i + 1, cmd);
                        }
                    }
                    Some(Command {
                        name: "which",
                        args,
                    }) => {
                        if args.is_empty() {
                            println!("smash: which: Too few arguments");
                            continue;
                        }

                        for arg in &args {
                            match which::which(arg) {
                                Ok(path) => println!("{}", path.display()),
                                Err(_) => println!("{}: not found", arg),
                            }
                        }
                    }
                    Some(Command {
                        name: "terminal", ..
                    }) => {
                        let terminal_size = classic::term::specs::size();
                        let terminal_kind = classic::term::specs::kind();

                        println!(
                            "Terminal: {} size {}x{}",
                            terminal_kind, terminal_size.width, terminal_size.height
                        );
                    }
                    Some(Command { name, args }) => {
                        // Check if command exists
                        let cmd_exists = which::which(name).is_ok();

                        if !cmd_exists {
                            println!("smash: {}: command not found", name);
                            continue;
                        }

                        // Execute command
                        let status = ProcessCommand::new(name).args(&args).status();

                        if let Err(e) = status {
                            println!("smash: {}: {}", name, e);
                        }
                    }
                    None => continue,
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue; // Go back to the prompt
            }
        }
    }

    Ok(())
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    let mut cursor_position = 0;
    let mut history_position: Option<usize> = None;
    let mut current_input = String::new();

    // Load history for up/down navigation
    let history = CommandHistory::new(1000).unwrap_or_else(|_| CommandHistory {
        entries: VecDeque::new(),
        max_size: 1000,
        history_file: PathBuf::from(".smash_history"),
    });

    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                // Handle Ctrl+C (interrupt)
                if code == KeyCode::Char('c') && modifiers.contains(event::KeyModifiers::CONTROL) {
                    println!("^C");
                    return Ok(String::new());
                }

                // Handle Ctrl+D (EOF) when input is empty
                if code == KeyCode::Char('d')
                    && modifiers.contains(event::KeyModifiers::CONTROL)
                    && input.is_empty()
                {
                    println!("exit");
                    return Ok("exit".to_string());
                }

                match code {
                    KeyCode::Enter => {
                        println!();
                        break;
                    }
                    KeyCode::Char(c) => {
                        input.insert(cursor_position, c);
                        cursor_position += 1;

                        // Save current edited command and reset history position
                        current_input = input.clone();
                        history_position = None;

                        // Redraw the line
                        print!("\r");
                        term::prompt::print_prompt()?;
                        print!("{}", input);

                        // Move cursor back to position if needed
                        if cursor_position < input.len() {
                            print!("\x1b[{}D", input.len() - cursor_position);
                        }

                        stdout().flush()?;
                    }
                    KeyCode::Backspace => {
                        if cursor_position > 0 {
                            input.remove(cursor_position - 1);
                            cursor_position -= 1;

                            // Save current edited command and reset history position
                            current_input = input.clone();
                            history_position = None;

                            // Redraw the line
                            print!("\r");
                            term::prompt::print_prompt()?;
                            print!("{}", input);

                            // Clear to the end of line in case the new string is shorter
                            print!("\x1b[K");

                            // Move cursor back to position if needed
                            if cursor_position < input.len() {
                                print!("\x1b[{}D", input.len() - cursor_position);
                            }

                            stdout().flush()?;
                        }
                    }
                    KeyCode::Delete => {
                        if cursor_position < input.len() {
                            input.remove(cursor_position);

                            // Save current edited command and reset history position
                            current_input = input.clone();
                            history_position = None;

                            // Redraw the line
                            print!("\r");
                            term::prompt::print_prompt()?;
                            print!("{}", input);

                            // Clear to the end of line in case the new string is shorter
                            print!("\x1b[K");

                            // Move cursor back to position
                            if cursor_position < input.len() {
                                print!("\x1b[{}D", input.len() - cursor_position);
                            }

                            stdout().flush()?;
                        }
                    }
                    KeyCode::Left => {
                        if cursor_position > 0 {
                            cursor_position -= 1;
                            print!("\x1b[D");
                            stdout().flush()?;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_position < input.len() {
                            cursor_position += 1;
                            print!("\x1b[C");
                            stdout().flush()?;
                        }
                    }
                    KeyCode::Up => {
                        // Navigate history upward
                        let history_len = history.len();
                        if history_len > 0 {
                            let new_pos = match history_position {
                                None => history_len - 1,
                                Some(pos) if pos > 0 => pos - 1,
                                _ => 0,
                            };

                            history_position = Some(new_pos);

                            if let Some(cmd) = history.get(new_pos) {
                                // Save current input before modifying if we're just starting to navigate
                                if history_position == Some(history_len - 1) {
                                    current_input = input.clone();
                                }

                                input = cmd.clone();
                                cursor_position = input.len();

                                // Redraw with history command
                                print!("\r");
                                term::prompt::print_prompt()?;
                                print!("{}\x1b[K", input);
                                stdout().flush()?;
                            }
                        }
                    }
                    KeyCode::Down => {
                        // Navigate history downward
                        if let Some(pos) = history_position {
                            if pos < history.len() - 1 {
                                let new_pos = pos + 1;
                                history_position = Some(new_pos);

                                if let Some(cmd) = history.get(new_pos) {
                                    input = cmd.clone();
                                    cursor_position = input.len();
                                }
                            } else {
                                // At the end of history, restore current input
                                input = current_input.clone();
                                cursor_position = input.len();
                                history_position = None;
                            }

                            // Redraw with new command
                            print!("\r");
                            term::prompt::print_prompt()?;
                            print!("{}\x1b[K", input);
                            stdout().flush()?;
                        }
                    }
                    KeyCode::Home => {
                        print!("\r");
                        term::prompt::print_prompt()?;
                        cursor_position = 0;
                        stdout().flush()?;
                    }
                    KeyCode::End => {
                        if cursor_position < input.len() {
                            print!("\x1b[{}C", input.len() - cursor_position);
                            cursor_position = input.len();
                            stdout().flush()?;
                        }
                    }
                    KeyCode::Tab => {
                        // Simple command completion
                        if input.contains(' ') {
                            // File path completion - TBD in future implementation
                        } else if !input.is_empty() {
                            // Command completion
                            let mut matches = Vec::new();

                            // Try common directories in PATH
                            for path_str in ["/usr/bin", "/bin", "/usr/local/bin"] {
                                let path = Path::new(path_str);
                                if path.exists() {
                                    if let Ok(entries) = fs::read_dir(path) {
                                        for entry in entries.filter_map(Result::ok) {
                                            if let Some(name) = entry.file_name().to_str() {
                                                if name.starts_with(&input) {
                                                    matches.push(name.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Also try builtin commands
                            for builtin in [
                                "cd", "pwd", "exit", "echo", "env", "clear", "history", "which",
                            ] {
                                if builtin.starts_with(&input) {
                                    matches.push(builtin.to_string());
                                }
                            }

                            if matches.len() == 1 {
                                // One match - complete the command
                                input = matches[0].clone();
                                cursor_position = input.len();

                                print!("\r");
                                term::prompt::print_prompt()?;
                                print!("{}", input);
                                stdout().flush()?;
                            } else if matches.len() > 1 {
                                // Multiple matches - show options
                                println!();
                                for m in matches {
                                    println!("{}", m);
                                }

                                print!("\r");
                                term::prompt::print_prompt()?;
                                print!("{}", input);
                                stdout().flush()?;
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Clear the input when Escape is pressed
                        input.clear();
                        cursor_position = 0;
                        print!("\r");
                        term::prompt::print_prompt()?;
                        print!("\x1b[K"); // Clear to end of line
                        stdout().flush()?;
                    }
                    _ => {}
                }
            }
            Event::Resize(_, _) => {
                // Redraw on terminal resize
                print!("\r");
                term::prompt::print_prompt()?;
                print!("{}", input);

                // Move cursor back to position if needed
                if cursor_position < input.len() {
                    print!("\x1b[{}D", input.len() - cursor_position);
                }

                stdout().flush()?;
            }
            _ => {}
        }
    }

    Ok(input)
}

fn init_posix() -> Result<(), Box<dyn Error>> {
    // Set environment variables for POSIX compliance
    if env::var("PATH").is_err() {
        env::set_var("PATH", "/usr/local/bin:/usr/bin:/bin");
    }

    // Set important POSIX variables if not already set
    if env::var("HOME").is_err() {
        if let Some(home) = std::env::home_dir() {
            env::set_var("HOME", home.to_string_lossy().to_string());
        }
    }

    if env::var("USER").is_err() {
        if let Ok(user) = env::var("LOGNAME") {
            env::set_var("USER", user);
        } else if let Ok(output) = ProcessCommand::new("whoami").output() {
            if output.status.success() {
                let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
                env::set_var("USER", user);
            }
        }
    }

    if env::var("SHELL").is_err() {
        env::set_var("SHELL", env::current_exe()?.to_string_lossy().to_string());
    }

    if env::var("PWD").is_err() {
        if let Ok(pwd) = env::current_dir() {
            env::set_var("PWD", pwd.to_string_lossy().to_string());
        }
    }

    Ok(())
}

fn read_line(input: &mut String) -> Option<Command> {
    if let Err(error) = stdin().read_line(input) {
        writeln!(stderr(), "Error: {}", error).unwrap();
        exit(1);
    }
    parse_command(input)
}

fn parse_command(input: &String) -> Option<Command> {
    // Safety check for empty or whitespace-only input
    if input.trim().is_empty() {
        return None;
    }

    let mut split_input = input.trim().split_whitespace();

    if let Some(name) = split_input.next() {
        let args = split_input.map(|arg| arg.to_string()).collect();
        Some(Command { name, args })
    } else {
        None
    }
}
