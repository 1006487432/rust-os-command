use ncurses::*;

const LEFTEDGE: i32 = 10;
const RIGHTEDGE: i32 = 30;
const ROW: i32 = 10;

fn main() {
    let message = "Hello";
    let blank = "     ";
    let mut dir = 1;
    let mut pos = LEFTEDGE;

    initscr();
    clear();

    loop {
        mvprintw(ROW, pos, message);    // draw string
        mvprintw(LINES() - 1, COLS() - 1, ""); // park the cursor
        refresh();   // show string
        std::thread::sleep(std::time::Duration::from_secs(1));
        mvprintw(ROW, pos, blank);    // erase string
        pos += dir;  // advance position

        if pos >= RIGHTEDGE {   // check for bounce
            dir = -1;
        }
        if pos <= LEFTEDGE {
            dir = 1;
        }
    }

    endwin();
}
