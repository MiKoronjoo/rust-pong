extern crate ncurses;

use ncurses::*;
mod structs;

pub fn run() {
    let key_s = 115;
    let key_w = 119;
    let key_q = 113;
    let key_up = 65;
    let key_down = 66;
    let mut game = structs::Game::new();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    game.show();
    // wmove(game.window, y / 2, x / 2);
    // addstr("██");

    loop {
        let key = getch();
        if key_down == key {
            game.down(2)
        } else if key_up == key {
            game.up(2)
        } else if key_s == key {
            game.down(1)
        } else if key_w == key {
            game.up(1)
        } else if key_q == key {
            break;
        }
        clear();
        game.show();
        // addstr(key.to_string().as_str());
    }

    /* Terminate ncurses. */
    endwin();
}
