extern crate ncurses;

use ncurses::*;

pub struct Board {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

pub struct Game {
    board: Board,
    p1: Player,
    p2: Player,
    window: *mut i8,
}

pub struct Player {
    x: i32,
    y: i32,
    len: i32,
}

impl Player {
    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn down(&mut self) {
        self.y += 1;
    }
}

impl Game {
    pub fn new() -> Self {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");
        let window = initscr();
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        getmaxyx(window, &mut y, &mut x);
        Self {
            board: Board {
                x0: 1,
                y0: 2,
                x1: x - 2,
                y1: y - 3,
            },
            p1: Player {
                x: 4,
                y: 3,
                len: y / 4,
            },
            p2: Player {
                x: x - 4,
                y: 3,
                len: y / 4,
            },
            window: window,
        }
    }

    pub fn up(&mut self, p: u8) {
        if p == 1 {
            self.p1.up();
        } else {
            self.p2.up();
        }
    }
    pub fn down(&mut self, p: u8) {
        if p == 1 {
            if self.board.y1 - self.board.y0 >= self.p1.len + self.p1.y {
                self.p1.down();
            }
        } else {
            if self.board.y1 - self.board.y0 >= self.p2.len + self.p2.y {
                self.p2.down();
            }
        }
    }

    pub fn show(&self) {
        let window = self.window;
        wmove(window, self.board.x0, self.board.y0);
        addstr("▛");
        for _ in 0..self.board.x1 - 3 {
            addstr("▀");
        }
        addstr("▜");
        for yt in self.board.y0..self.board.y1 + 1 {
            wmove(window, yt, 2);
            addstr("▌");
            wmove(window, yt, self.board.x1);
            addstr("▐");
        }
        wmove(window, self.board.y1 + 1, self.board.x0 + 1);
        addstr("▙");
        for _ in 0..self.board.x1 - 3 {
            addstr("▄");
        }
        addstr("▟");

        for y in self.p1.y..self.p1.y + self.p1.len {
            wmove(window, y + 2, self.p1.x);
            addstr("█");
        }

        for y in self.p2.y..self.p2.y + self.p2.len {
            wmove(window, y + 2, self.p2.x);
            addstr("█");
        }
    }
}
