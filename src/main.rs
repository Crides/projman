extern crate regex;
extern crate ncurses;
extern crate pancurses;
use pancurses::*;
#[macro_use]
extern crate lazy_static;
use std::process::Command;

mod ui;
use ui::*;

fn main() {
    let term = TermWin::new();
    if !term.check_size() {
        term.finish();
        return;
    }

    term.draw_border();
    let output = Command::new("git")
            .arg("-c")
            .arg("color.ui=always")
            .arg("status")
            .output()
            .unwrap();
    TermWin::printwc(&term.rpad, &String::from_utf8_lossy(&output.stdout));
    loop {
        match term.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    break;
                }
            },
            _ => (),
        }
    }
    term.finish();
}
