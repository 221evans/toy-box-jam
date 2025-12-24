use raylib::prelude::*;

pub struct Bullet {
    texture : Texture2D,
    source_rect : Rectangle,
    pub dest_rect : Rectangle,
    origin : Vector2,
    speed : f32,
    pub pos_x : f32,
    pub pos_y : f32,
    pub is_dead : bool,

}

impl Bullet {

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, pos_x : f32, pos_y : f32) -> Self {

        let bullet_texture = rl
            .load_texture(thread,"assets/sprites.png")
            .expect("Error loading bullet texture");

        Bullet {
            texture : bullet_texture,
            source_rect : Rectangle::new(896.0,256.0,64.0,64.0),
            dest_rect : Rectangle::new(pos_x,pos_y,32.0,32.0),
            origin : Vector2::new(0.0,0.0),
            speed : 300.0,
            pos_x,
            pos_y,
            is_dead : false,
        }
    }

    pub fn update(&mut self, delta_time : f32, rl : &mut RaylibHandle) {

        self.dest_rect.y -= self.speed * delta_time;

        self.pos_y = self.dest_rect.y;
        self.pos_x = self.dest_rect.x;
        
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
}