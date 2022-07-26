extern crate ncurses;
use itertools::Itertools;
use ncurses::*;
mod structs;

pub fn run() {
    let key_s = 115;
    let key_w = 119;
    let key_q = 113;
    let key_o = 111;
    let key_l = 108;
    let mut game = structs::Game::new();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    game.show();
    let mut quit = false;
    timeout(10);
    loop {
        let key = [getch(), getch(), getch(), getch()];
        for &k in key.iter().unique() {
            if key_q == k {
                quit = true;
                break;
            }
            if key_l == k {
                game.down(2);
            } else if key_o == k {
                game.up(2);
            } else if key_s == k {
                game.down(1);
            } else if key_w == k {
                game.up(1);
            }
        }
        if quit {
            break;
        }
        // if key[0] == -1 && key[1] == -1 && key[2] == -1 && key[3] == -1 {
        //     continue;
        // }
        clear();
        game.ball.mov();
        game.ball_impact();
        game.show();

        // for i in 0..4 {
        //     wmove(game.window, 2, 4);
        //     addstr((key[i] as u8 as char).to_string().as_str());
        //     addstr("_");
        // }
    }

    /* Terminate ncurses. */
    endwin();
}
