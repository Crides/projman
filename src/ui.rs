use std::io::{stdout, Write};
use pancurses::*;
use regex::Regex;
use std::ptr::null_mut;

pub enum TermMode {
    Cmd,
    Norm,
}

pub struct TermWin {
    win: Window,
    mode: TermMode,
    vdiv_pos: i32,

    pub lpad: Window,
    pub rpad: Window,
}

impl TermWin {
    pub const MIN_ROW: i32 = 12;
    pub const MIN_COL: i32 = 50;

    pub fn new() -> Self {
        let window = initscr();
        window.keypad(true);
        mousemask(ALL_MOUSE_EVENTS, null_mut()); // Listen to all mouse events
        noecho();
        curs_set(0);
        print!("[?1002h[?1006h");           // Turn on button event tracking; extended mode

        // Initialize colors
        start_color();
        use_default_colors();
        init_pair(0, -1, -1);
        for i in 1..8 {
            init_pair(i, i, -1);
        }

        let vdiv_pos = window.get_max_x() / 3 * 2;
        let lpad = window.subwin(window.get_max_y() - 4, vdiv_pos - 1, 2, 0).unwrap();
        let rpad = window.subwin(window.get_max_y() - 4, window.get_max_x() - vdiv_pos - 1, 2, vdiv_pos + 1).unwrap();
        Self {
            win: window,
            mode: TermMode::Norm,
            vdiv_pos: vdiv_pos,
            
            lpad: lpad,//newpad(100, 100),
            rpad: rpad,
        }
    }

    pub fn finish(&self) {
        print!("[?1002l[?1006l");
        endwin();
    }

    pub fn check_size(&self) -> bool {
        let size = self.win.get_max_yx();
        if size == (0, 0) {
            eprintln!("Cannot determin size of current window!");
            return false;
        }

        if size.0 < TermWin::MIN_ROW || size.1 < TermWin::MIN_COL {
            eprintln!("Terminal too small to be useful!");
            eprintln!("Current size: {} rows, {} cols.", size.0, size.1);
            return false;
        }

        true
    }

    pub fn draw_border(&self) {
        self.win.mvprintw(0, 0, "    ");
        self.win.attron(A_BOLD);
        self.win.printw("Projman v0.1.0");
        self.win.mv(1, 0);
        self.win.hline('-', self.win.get_max_x());

        self.win.mv(2, self.vdiv_pos);
        self.win.vline('|', self.win.get_max_y() - 4);
        self.win.mvprintw(1, self.vdiv_pos, "+");
        self.win.attrset(A_NORMAL);
    }
    
    pub fn getch(&self) -> Option<Input> {
        self.win.getch()
    }

    pub fn printwc(w: &Window, s: &str) {        // Print string to window with correct colors
        lazy_static! {
            static ref CC_PAT: Regex = Regex::new(r"\[\d*m").unwrap();
        }
        let mut chunks = CC_PAT.split(s);
        let mut codes = CC_PAT.find_iter(s);
        w.printw(chunks.next().unwrap());
        for (i, chunk) in chunks.enumerate() {
            let code = codes.next().unwrap();
            let color = code.as_str().get(3..4)
                    .unwrap_or("")
                    .parse::<u8>()
                    .unwrap_or(0);
            w.attrset(ColorPair(color));
            w.addstr(chunk);
        }
    }

    pub fn refresh(&self) {
        self.lpad.prefresh(0, 0,
                           2, 0,
                           self.win.get_max_y() - 2, self.vdiv_pos - 1);    // FIXME
        self.rpad.prefresh(0, 0,
                           2, self.vdiv_pos + 1,
                           self.win.get_max_y() - 2, self.win.get_max_x()); // FIXME
        self.win.refresh();
    }
}

pub fn flush_stdout() {
    stdout().flush().unwrap();
}
