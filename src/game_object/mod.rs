use opengl_graphics::{ GlGraphics };
use graphics::{ Context };

mod position;
mod player;
mod astroid;

pub use position::Position;
pub use player::Player;
pub use astroid::Astroid;

pub trait GameObject {
    fn position(&self) -> &Position;
    fn radius(&self) -> f64;
    fn render(&self, c: &Context, g: &mut GlGraphics);
    fn update(&mut self, dt: f64);
    fn collides(&self, other: &GameObject) -> bool {
        let x2 = self.position().x - other.position().x;
        let y2 = self.position().y - other.position().y;
        let sum = x2.powf(2.0) + y2.powf(2.0);
        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();
        r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0)
    }
}

impl GameObject for Player {
    fn position(&self) -> &Position{ &self.position }
    fn radius(&self) -> f64 { self.size / 2.0 }
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let radius = self.radius();

        let body_transform = ctxt.transform
            .trans(self.position.x, self.position.y)
            .trans(-radius, radius);

        let gun_transform = ctxt.transform
            .trans(self.position.x, self.position.y)
            .trans(0.0, 2.0 * radius)
            .rot_rad(self.angle)
            .trans(0.0, -0.5 * radius);

        rectangle(WHITE, [0.0, 0.0, self.size, radius], gun_transform, gl);
        rectangle(WHITE, [0.0, 0.0, self.size, self.size], body_transform, gl);
    }
    fn update(&mut self, dt: f64) {
        const MOVESCALAR: f64 = 5.0;
        self.position.x += dt * self.velocity.x * MOVESCALAR;
        self.position.y += dt * self.velocity.y * MOVESCALAR;
    }
}

impl GameObject for Astroid {
    fn position(&self) -> &Position{ &self.position }
    fn radius(&self) -> f64 { self.size / 2.0 }
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let radius = self.radius();

        let body_shape = Rectangle::new_round_border(WHITE, radius, 1.0);

        let body_transform = ctxt.transform
            .trans(self.position.x, self.position.y)
            .trans(-radius, radius);

        body_shape.draw([0.0, 0.0, 1.0, 1.0], &ctxt.draw_state, body_transform, gl);
    }
    fn update(&mut self, dt: f64) {
        const MOVESCALAR: f64 = 1.0;
        self.position.x += dt * self.velocity.x * MOVESCALAR;
        self.position.y += dt * self.velocity.y * MOVESCALAR;
    }
}