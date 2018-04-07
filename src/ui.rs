use nix::sys::termios::*;
use nix::libc::{VMIN, VTIME, getchar};

pub enum TermMode {
    Cmd,
    Norm,
}

#[derive(Debug)]
pub struct WinSize {
    row: u16,
    col: u16,
    xpixel: u16,
    ypixel: u16,
}

impl WinSize {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            xpixel: 0,
            ypixel: 0,
        }
    }
}

pub struct TermWin {
    old_state: Termios,
    mode: TermMode,
    pub size: WinSize,
}

ioctl!(bad read _term_size with 0x5413; WinSize);

impl TermWin {
    pub fn new() -> Self {
        let old_state = tcgetattr(0).unwrap();
        let mut win_size = WinSize::new();
        unsafe {
            _term_size(0, &mut win_size);
        }

        Self {
            old_state: old_state,
            mode: TermMode::Norm,
            size: win_size,
        }
    }

    pub fn init(&self) {
        print!("[s");             // Store cursor position
        print!("[?1049h");        // Store window in buffer
        print!("[2J");            // Clear screen

        let mut new_state = self.old_state.clone();
        new_state.local_flags.remove(LocalFlags::ICANON);
        new_state.local_flags.remove(LocalFlags::ECHO);
        new_state.control_chars[VMIN] = 1;
        new_state.control_chars[VTIME] = 0;
        tcsetattr(0, SetArg::TCSANOW, &new_state);
    }

    pub fn finish(&self) {
        tcsetattr(0, SetArg::TCSANOW, &self.old_state);

        println!("[u");
        print!("[?1049l");
    }

    pub fn check_size(&self) -> Option<()> {
        if self.size.row == 0 || self.size.col == 0 {
            eprintln!("Cannot determin size of current window!");
            return None;
        }

        if self.size.row < 10 || self.size.col < 50 {
            eprintln!("Terminal too small to be useful!");
            eprintln!("Current size: {} rows, {} cols.", self.size.row, self.size.col);
            return None;
        }

        Some(())
    }

    pub fn getch() -> char {
        unsafe {
            getchar() as u8 as char
        }
    }
}
