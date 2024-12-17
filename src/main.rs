use clap::{arg, parser::Values, Command};
use crossterm::{
    cursor,
    event::{self, DisableBracketedPaste, DisableFocusChange},
    execute,
    terminal::{self, DisableLineWrap},
    QueueableCommand,
};
use std::{
    io::{stdout, Write},
    process::exit,
    simd::LaneCount,
};
use terminal::event::{Event, KeyCode, KeyEvent, MouseEventKind};

static KOSH_APP_NAME: &str = env!("CARGO_PKG_NAME");
static KOSH_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(KOSH_APP_NAME)
        .version(KOSH_APP_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_version_flag(true)
        .args(&[arg!(--posix "Run in a POSIX compatible mode")])
        .args(&[arg!(-v --version "Prints version information")])
        .get_matches();

    if matches.get_flag("version") {
        println!("{} {}", KOSH_APP_NAME, KOSH_APP_VERSION);
        exit(0);
    }

    terminal::enable_raw_mode().expect("Terminal does not handle raw mode");

    let mut output = stdout();

    let u = output
        .queue(terminal::EnableLineWrap)?
        .queue(event::EnableFocusChange)?
        .queue(event::EnableBracketedPaste)?;

    output
        .queue(cursor::Show)?
        .queue(cursor::SetCursorStyle::SteadyBar)?
        .flush()?;

    // loop{
    //     let event = crossterm::event::read()?;
    //     let mut input = String::new();
    //     match event {
    //         Event::FocusGained => execute!(output, cursor::EnableBlinking)?,
    //         Event::FocusLost => execute!(output, cursor::DisableBlinking)?,
    //         Event::Key(event) => {
    //             if !event.modifiers.is_empty() {
    //                 print!("{:?} ", event.modifiers);
    //             }
    //             match event.code {
    //                 KeyCode::Esc => {
    //                     break;
    //                 }
    //                 KeyCode::Char(char) => {
    //                     print!("{}", char);
    //                     input.push(char);
    //                     output.flush()?;
    //                 }
    //                 KeyCode::Enter => {
    //                     output
    //                         .queue(cursor::MoveToNextLine(1))?
    //                         // .queue(cursor::MoveToColumn(0))?
    //                         .flush()?;
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         Event::Mouse(event) => {
    //             output.queue(cursor::MoveTo(event.column, event.row))?;
    //             if event.kind == event::MouseEventKind::Up(event::MouseButton::Left) {
    //                 output.write("Left".as_bytes())?;
    //             } else if event.kind == event::MouseEventKind::Up(event::MouseButton::Right) {
    //                 output.write("Right".as_bytes())?;
    //             }
    //             output.flush()?
    //         }
    //         Event::Paste(data) => println!("{:?}", data),
    //         Event::Resize(width, height) => println!("New size {}x{}", width, height),
    //     }
    // }

    kosh::run_loop(matches)?;

    execute!(
        output,
        DisableLineWrap,
        // DisableMouseCapture,
        DisableFocusChange,
        DisableBracketedPaste
    )?;

    if let Ok(true) = terminal::is_raw_mode_enabled() {
        terminal::disable_raw_mode()?;
    };
    ()
}

fn posix_mode() -> _ {
    // Set the terminal to run in posix mode
    todo!()
}
