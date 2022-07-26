extern crate ncurses;
use ncurses::*;
use rand::prelude::*;
use std::f32::consts::PI;

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
    pub ball: Ball,
    pub window: *mut i8,
}

pub struct Player {
    x: i32,
    y: i32,
    len: i32,
    pub score: u16,
}

pub struct Ball {
    x: f32,
    y: f32,
    dig: f32,
    speed: f32,
}

impl Ball {
    pub fn mov(&mut self) {
        self.y -= self.dig.sin() * self.speed;
        self.x += self.dig.cos() * self.speed;
    }

    pub fn x(&mut self) -> i32 {
        self.x as i32
    }

    pub fn y(&mut self) -> i32 {
        self.y as i32
    }
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
                score: 0,
            },
            p2: Player {
                x: x - 4,
                y: 3,
                len: y / 4,
                score: 0,
            },
            ball: Ball {
                x: (x / 2) as f32,
                y: (y / 2) as f32,
                dig: 0.0,
                speed: 1.0,
            },
            window: window,
        }
    }

    pub fn ball_impact(&mut self) {
        let mut rng = rand::thread_rng();
        let randnum: f32 = rng.gen();
        if (self.ball.x() == self.p1.x || self.ball.x() - 1 == self.p1.x)
            && (self.ball.y() - 2 >= self.p1.y && self.ball.y() - 2 < self.p1.y + self.p1.len)
        {
            self.ball.dig = PI - self.ball.dig;
            self.ball.dig += (self.ball.dig / 8.0) * (0.5 - randnum);
        } else if (self.ball.x() + 1 == self.p2.x || self.ball.x() == self.p2.x)
            && (self.ball.y() - 2 >= self.p2.y && self.ball.y() - 2 < self.p2.y + self.p2.len)
        {
            self.ball.dig = PI - self.ball.dig;
            self.ball.dig += (self.ball.dig / 8.0) * (0.5 - randnum);
        } else if self.ball.y() <= 1 || self.ball.y() > self.board.y1 {
            self.ball.dig *= -1.0;
        } else if self.ball.x() <= 0 {
            self.p2.score += 1;
            self.ball.x = (self.board.x1 / 2) as f32;
            self.ball.y = (self.board.y1 / 2) as f32;
            self.ball.dig = 0.0;
        } else if self.ball.x() >= self.board.x1 {
            self.p1.score += 1;
            self.ball.x = (self.board.x1 / 2) as f32;
            self.ball.y = (self.board.y1 / 2) as f32;
            self.ball.dig = PI;
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
        wmove(window, 0, 3);
        addstr(self.p1.score.to_string().as_str());
        wmove(
            window,
            0,
            self.board.x1 - self.p2.score.to_string().len() as i32,
        );
        addstr(self.p2.score.to_string().as_str());
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

        wmove(window, self.ball.y as i32, self.ball.x as i32);
        addstr("██");

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
