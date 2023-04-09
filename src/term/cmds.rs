pub enum BuiltinCommandType {
    Clear,
    Exit,
    History,
    ChangeDirectory,
    List,
}

pub struct BuiltinCommand {
    pub name: String,
    pub function: fn(&[String]) -> Result<(), String>,
}

impl BuiltinCommand {
    pub fn new(name: &str, function: fn(&[String]) -> Result<(), String>) -> BuiltinCommand {
        BuiltinCommand {
            name: name.to_string(),
            function,
        }
    }

    pub fn execute(&self, args: &[String]) -> Result<(), String> {
        (self.function)(args)
    }
}

pub fn builtin_command(name: &str) -> Option<&'static BuiltinCommand> {
    for command in BUILTINS {
        if command.name == name {
            return Some(command);
        }
    }

    None
}

pub fn builtin_execute(name: &str, args: &[String]) -> Result<(), String> {
    if let Some(command) = builtin_command(name) {
        (command.function)(args)
    } else {
        Err(format!("{}: command not found", name))
    }
}

pub enum Command {
    Builtin(BuiltinCommand),
    External(String),
}

impl Command {
    pub fn execute(command: Command, args: &[String]) -> Result<(), String> {
        match command {
            Command::Builtin(command) => command.execute(args),
            Command::External(command) => external_execute(command, args),
        }
    }
}

pub enum BuiltinCommandDefinitions {
    // BuiltinCommand("clear", clear),
}

pub fn clear(args: &[String]) -> Result<(), String> {
    if args.len() > 1 {
        return Err("clear: too many arguments".to_string());
    }

    print!("{}[2J", 27 as char);
    Ok(())
}

pub fn execute_builtin(command: &str, args: &mut dyn Iterator<Item = &str>) {
    match command {
        Some("history") => {
            // Reads history File
            let home = std::env::var("HOME").unwrap();
            // Searches for the history file in the home folder
            let history_file = std::path::Path::new(&home).join(".kosh_history");
            let history = std::fs::read_to_string(history_file).unwrap();
            println!("{}", history);
        }
        Some("cd") => {
            let path = args.next().unwrap_or("");
            std::env::set_current_dir(path).unwrap();
        }
        Some("remove") => {
            let path = args.next().unwrap_or("");
            std::fs::remove_file(path).unwrap();
        }
        Some("create-dir") => {
            let path = args.next().unwrap_or("");
            std::fs::create_dir(path).unwrap();
        }
        Some("remove-dir") => {
            let path = args.next().unwrap_or("");
            std::fs::remove_dir(path).unwrap();
        }
        Some("create") => {
            let path = args.next().unwrap_or("");
            std::fs::File::create(path).unwrap();
        }
        Some("move") => {
            let from = args.next().unwrap_or("");
            let to = args.next().unwrap_or("");

            std::fs::rename(from, to).unwrap();
        }
        Some("copy") => {
            let from = args.next().unwrap_or("");
            let to = args.next().unwrap_or("");

            std::fs::copy(from, to).unwrap();
        }
        Some("find") => {
            let path = args.next().unwrap_or("");
            let mut paths = std::fs::read_dir(path).unwrap();

            while let Some(Ok(path)) = paths.next() {
                println!("{}", path.path().display());
            }
        }
        Some("whoami") => println!("{}", std::env::var("USER").unwrap()),
        Some("list") => {
            let path = args.next().unwrap_or(".");

            let dir = std::fs::read_dir(path).unwrap();

            for entry in dir {
                let entry = entry.unwrap();
                let path = entry.path();

                println!("{}", path.display());
            }
        }
    }
}
