use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

pub struct Player {
    pub pos_x : f32,
    pub pos_y : f32,
    source_rect : Rectangle,
    dest_rect : Rectangle,
    player_texture : Texture2D,
    origin: Vector2,
    speed : f32
}

impl Player {
    pub fn new(rl : &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let pos_x = 50.0;
        let pos_y = 400.0;

        let player_texture = rl
            .load_texture(thread,"assets/sprites.png")
            .expect("Error loading player texture");


        Player{
            pos_x,
            pos_y,
            source_rect : Rectangle::new(768.0,256.0,64.0,64.0),
            dest_rect : Rectangle::new(pos_x,pos_y,32.0,32.0),
            player_texture,
            origin : Vector2::new(0.0,0.0),
            speed : 100.0,

        }
    }

    pub fn update(&mut self, delta_time : f32, rl : &mut RaylibHandle) {

        self.dest_rect.x = self.pos_x;
        self.dest_rect.y = self.pos_y;

        if rl.is_key_down(KEY_A) {
            self.pos_x -= self.speed * delta_time;
        }
        else if rl.is_key_down(KEY_D) {
            self.pos_x += self.speed * delta_time;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            &self.player_texture,
            self.source_rect,
            self.dest_rect,
            self.origin,
            0.0,
            Color::WHITE
        )


    }
}