use raylib::prelude::*;
use crate::player::Player;

mod player;
mod alien;
mod bullet;
mod game;



fn main() {
    let (mut rl, thread) = init()
        .size(640, 480)
        .title("Toy Box Jam")
        .build();

    let mut game = game::Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        // Update here
        game.update(delta_time, &mut rl, &thread);


        let mut d = rl.begin_drawing(&thread);


        d.clear_background(Color::BLACK);

        // Draw here
        game.draw(&mut d);

    }
}
