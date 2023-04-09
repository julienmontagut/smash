use windows_sys::Win32::System::Console;

struct Terminal {
    mode: u32,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal { mode: 0 }
    }

    pub fn init_byte_mode(&self) {
        unsafe {
            Console::GetConsoleMode(Console::STD_INPUT_HANDLE, &mut mode);
            Console::SetConsoleMode(Console::STD_INPUT_HANDLE, mode & !Console::ENABLE_PROCESSED_INPUT);
        }
    }

    pub fn reset_mode(&self) {
        unsafe {
            Console::SetConsoleMode(Console::STD_INPUT_HANDLE, mode);
        }
    }
}

impl TerminalMode for Terminal {
    fn init(&self) {
        self.init_byte_mode();
    }

    fn reset(&self) {
        self.reset_mode();
    }
}
