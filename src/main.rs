use clap::{arg, Command as ClapCommand};
use crossterm::{
    cursor, event::{self, DisableBracketedPaste, DisableFocusChange}, execute, terminal::{self, DisableLineWrap}, QueueableCommand
};
use std::{
    error::Error,
    io::{self, stdout, Write},
};
// use terminal::event::{Event, KeyCode, KeyEvent, MouseEventKind};

mod dirs;

static SMASH_APP_NAME: &str = env!("CARGO_PKG_NAME");
static SMASH_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    // Set up panic handler to ensure terminal is reset on panic
    std::panic::set_hook(Box::new(|panic_info| {
        // Clean up terminal on panic
        let _ = terminal::disable_raw_mode();
        let mut stdout = stdout();
        let _ = execute!(
            stdout,
            cursor::Show,
            DisableLineWrap,
            DisableFocusChange,
            DisableBracketedPaste
        );
        eprintln!("Error: {}", panic_info);
    }));

    let matches = ClapCommand::new(SMASH_APP_NAME)
        .version(SMASH_APP_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args(&[arg!(--posix "Run in a POSIX compatible mode")])
        .get_matches();
    
    // Enable raw mode with better error handling
    if let Err(e) = terminal::enable_raw_mode() {
        eprintln!("Failed to set up terminal: {}", e);
        eprintln!("The shell may not function correctly.");
    }

    let mut output = stdout();

    // Set up terminal with better error handling
    let terminal_result = || -> Result<(), io::Error> {
        output.queue(terminal::EnableLineWrap)?;
        output.queue(event::EnableFocusChange)?;
        output.queue(event::EnableBracketedPaste)?;
        output.queue(cursor::Show)?;
        output.queue(cursor::SetCursorStyle::SteadyBar)?;
        output.flush()?;
        Ok(())
    }();
    
    if let Err(e) = terminal_result {
        eprintln!("Warning: Failed to configure terminal: {}", e);
    }

    // Setup trap for SIGINT to properly handle Ctrl+C
    #[cfg(unix)]
    {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        
        if let Err(e) = ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            // Just print a new line when Ctrl+C is pressed
            println!();
        }) {
            eprintln!("Warning: Could not set Ctrl-C handler: {}", e);
        }
    }

    // Run the shell, then handle cleanup regardless of result
    let result = smash::run_loop(matches);

    // Always clean up terminal state regardless of success or error
    let cleanup_result = execute!(
        output,
        DisableLineWrap,
        DisableFocusChange,
        DisableBracketedPaste
    );

    if let Err(e) = cleanup_result {
        eprintln!("Warning: Failed to reset terminal state: {}", e);
    }

    if let Ok(true) = terminal::is_raw_mode_enabled() {
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("Warning: Failed to disable raw mode: {}", e);
        }
    }

    // Return the result, or a generic error if there was a panic
    result
}

fn posix_mode() -> Result<(), Box<dyn Error>> {
    // Set the terminal to run in posix mode
    todo!()
}
