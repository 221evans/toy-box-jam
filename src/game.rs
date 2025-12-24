use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;
use crate::player::Player;
use crate::alien::Alien;
use crate::bullet::Bullet;

pub struct Game {
    player : Player,
    alien : Vec<Alien>,
    bullets : Vec<Bullet>,
    alien_direction : f32,

}

impl Game {

    pub fn new(rl : &mut RaylibHandle, thread : &RaylibThread) -> Self {

        let mut aliens  = Vec::new();

        for row in 0..5 {
            for col in 0..11 {
                aliens.push(Alien::new(
                    rl,
                    thread,
                    col as f32 * 40.0 + 50.0,
                    row as f32 * 40.0 + 50.0
                ))
            }
        }

        Game{
            player : Player::new(rl,thread),
            alien : aliens,
            bullets : Vec::new(),
            alien_direction : 1.0,
        }
    }

    pub fn update(&mut self, delta_time : f32, rl : &mut RaylibHandle,thread : &RaylibThread) {


        let hit_edge = self.alien.iter().any(|a|
            a.pos_x <= 50.0 || a.pos_x >= 600.0
        );

        if hit_edge {
            self.alien_direction *= -1.0;
        }



        for alien in &mut self.alien {
            alien.pos_x += self.alien_direction * 50.0 * delta_time;
            alien.update(delta_time, rl);

        }
        self.player.update(delta_time, rl);

        if rl.is_key_pressed(KEY_SPACE) {
            self.bullets.push(Bullet::new(rl,thread, self.player.pos_x, self.player.pos_y + -40.0))
        }

        for bullet in &mut self.bullets {
            bullet.update(delta_time, rl);
        }


        for bullet in &mut self.bullets {
            for alien in &mut self.alien {
                if check_collision(bullet.dest_rect, alien.dest_rect) {
                    alien.is_dead = true;
                    bullet.is_dead = true;

                }
            }
        }



        // Remove off-screen bullets
        self.bullets.retain(|b| b.pos_y > 0.0);

        // Remove dead aliens
        self.alien.retain(|a| !a.is_dead);

        // Remove dead bullets
        self.bullets.retain(|b| !b.is_dead);



    }

    pub fn draw(&mut self, d : &mut RaylibDrawHandle) {

        self.player.draw(d);

        for bullet in &mut self.bullets {
            bullet.draw(d);
        }

        for alien in &mut self.alien {
            alien.draw(d);
        }
    }
}

fn check_collision(rect1: Rectangle, rect2: Rectangle) -> bool {
    rect1.x < rect2.x + rect2.width
        && rect1.x + rect1.width > rect2.x
        && rect1.y < rect2.y + rect2.height
        && rect1.y + rect1.height > rect2.y
}