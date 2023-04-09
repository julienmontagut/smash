use std::{
    borrow::{Borrow, Cow},
    io::{stdin, Stdin},
    os::fd::AsRawFd,
};

pub struct Terminal {
    input: Stdin,
    attributes: libc::termios,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            input: stdin(),
            attributes: unsafe { std::mem::zeroed() },
        }
    }

    pub fn set_raw_mode(&self) {
        let mut attributes = self.read_attributes();
        attributes.c_iflag &= !(libc::IGNBRK
            | libc::BRKINT
            | libc::PARMRK
            | libc::ISTRIP
            | libc::INLCR
            | libc::IGNCR
            | libc::ICRNL
            | libc::IXON);
        attributes.c_oflag &= !libc::OPOST;
        attributes.c_lflag &=
            !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
        attributes.c_cflag &= !(libc::CSIZE | libc::PARENB);
        attributes.c_cflag |= libc::CS8;
        self.write_attributes(&mut attributes);
    }

    pub fn read_attributes(&self) -> libc::termios {
        let mut attributes = unsafe { std::mem::zeroed() };
        unsafe {
            libc::tcgetattr(self.input.as_raw_fd(), &mut attributes);
        }
        attributes
    }

    pub fn write_attributes(&self, &attributes: &libc::termios) {
        unsafe {
            libc::tcsetattr(
                self.input.as_raw_fd(),
                UpdateMode::Immediate.to_raw(),
                &attributes,
            );
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UpdateMode {
    Immediate = libc::TCSANOW,
    Drain = libc::TCSADRAIN,
    Flush = libc::TCSAFLUSH,
}

impl UpdateMode {
    pub fn to_raw(&self) -> libc::c_int {
        match self {
            UpdateMode::Immediate => libc::TCSANOW,
            UpdateMode::Drain => libc::TCSADRAIN,
            UpdateMode::Flush => libc::TCSAFLUSH,
        }
    }
}
