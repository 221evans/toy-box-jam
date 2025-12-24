use raylib::prelude::*;

enum MovedDirection {
    Left,
    Right
}

pub struct Alien {
    pub(crate) pos_x : f32,
    pos_y : f32,
    texture : Texture2D,
    origin : Vector2,
    source_rect : Rectangle,
    pub dest_rect : Rectangle,
    speed : f32,
    direction : MovedDirection,
    pub is_dead : bool,
}

impl Alien {

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, pos_x : f32, pos_y : f32) -> Self {

        let alien_texture = rl
            .load_texture(thread,"assets/sprites.png")
            .expect("Error loading alien texture");

        Alien{
            pos_x,
            pos_y,
            texture : alien_texture,
            origin : Vector2::new(0.0,0.0),
            source_rect : Rectangle::new(704.0,448.0,64.0,64.0),
            dest_rect : Rectangle::new(pos_x,pos_y,32.0,32.0),
            speed : 50.0,
            direction : MovedDirection::Right,
            is_dead : false,
        }
    }

    pub fn update(&mut self, delta_time : f32, rl : &mut RaylibHandle) {

        self.dest_rect.x = self.pos_x;
        self.dest_rect.y = self.pos_y;

        // if self.pos_x >= 600.0 {
        //     self.direction = MovedDirection::Left;
        // }
        // else if self.pos_x <= 10.0 {
        //     self.direction = MovedDirection::Right;
        // }

        // self.handle_move_direction(delta_time)
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            &self.texture,
            self.source_rect,
            self.dest_rect,
            self.origin,
             0.0,
            Color::WHITE
        )
    }

    fn handle_move_direction(&mut self, delta_time : f32) {
        match self.direction {
            MovedDirection::Left => self.pos_x -= self.speed * delta_time,
            MovedDirection::Right => self.pos_x += self.speed * delta_time,
        }
    }

}
