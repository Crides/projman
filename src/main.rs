#[macro_use]
extern crate nix;
mod ui;
use ui::*;

fn main() {
    let mut term = TermWin::new();
    if let None = term.check_size() {
        return;
    }

    term.init();
    println!("{:?}", term.size);
    loop {
        let ch = TermWin::getch();
        if ch == 'q' {
            break;
        }
    }
    term.finish();
}
