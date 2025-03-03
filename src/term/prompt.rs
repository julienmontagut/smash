use crossterm::{
    style::{self, Color, Stylize},
    terminal,
    QueueableCommand,
};
use std::env;
use std::io::{self, stdout, Write};

pub fn print_prompt() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let home_dir = env::var("HOME").unwrap_or_default();
    let username = env::var("USER").unwrap_or_else(|_| String::from("user"));
    let hostname = hostname::get()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|_| String::from("localhost"));

    // Get current directory, with ~ replacing $HOME
    let relative_path = if current_dir.starts_with(&home_dir) {
        current_dir
            .strip_prefix(&home_dir)
            .map(|p| {
                if p.as_os_str().is_empty() {
                    "~".to_string()
                } else {
                    format!("~/{}", p.display())
                }
            })
            .unwrap_or_else(|_| current_dir.display().to_string())
    } else {
        current_dir.display().to_string()
    };

    // Get git branch if in a git repository, with better error handling
    let git_part = match get_git_branch() {
        Ok(branch) if !branch.is_empty() => format!(" [{}]", branch).with(Color::Green).to_string(),
        _ => String::new(),
    };

    // Format timestamp
    let timestamp = format_timestamp();
    
    // Get terminal width to ensure prompt fits
    let (width, _) = terminal::size()?;
    
    // Build the prompt parts with colors
    let user_host = format!("{}@{}", username, hostname).with(Color::Blue);
    let path_part = relative_path.clone().with(Color::Magenta);
    let timestamp_part = timestamp.clone().with(Color::DarkGrey);
    
    // Calculate the length of the visible prompt text (without ANSI codes)
    let user_host_len = username.len() + hostname.len() + 1; // +1 for @
    let path_len = relative_path.len();
    // Get git branch length from git_part
    let git_part_len = git_part.len();
    let timestamp_len = timestamp.len();
    
    // If the prompt would be too long, show a more compact version
    let mut output = stdout();
    
    if user_host_len + path_len + git_part_len + timestamp_len + 4 > width as usize {
        // Compact prompt
        let dir_name = current_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "?".to_string());
            
        output
            .queue(style::PrintStyledContent(user_host))?
            .queue(style::Print(":"))?
            .queue(style::PrintStyledContent(dir_name.with(Color::Magenta)))?
            .queue(style::Print(git_part))?
            .queue(style::Print(" $ "))?;
    } else {
        // Full prompt
        output
            .queue(style::PrintStyledContent(timestamp_part))?
            .queue(style::Print(" "))?
            .queue(style::PrintStyledContent(user_host))?
            .queue(style::Print(":"))?
            .queue(style::PrintStyledContent(path_part))?
            .queue(style::Print(git_part))?
            .queue(style::Print(" $ "))?;
    }
    
    output.flush()?;
    Ok(())
}

fn get_current_dir_string() -> io::Result<String> {
    let path = env::current_dir()?;
    match path.to_str() {
        Some(p) => Ok(p.to_string()),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to convert path to string",
        )),
    }
}

fn get_git_branch() -> io::Result<String> {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output();
        
    match output {
        Ok(output) if output.status.success() => {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(branch)
        },
        _ => Ok(String::new()),
    }
}

fn format_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
        let secs = time.as_secs();
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        String::from("--:--:--")
    }
}
