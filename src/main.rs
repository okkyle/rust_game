use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod game_object;

use game_object::GameObject;
use game_object::Position;
use game_object::Player;

// structs

pub struct App {
    gl: GlGraphics,
    player: Player,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let player = &self.player;

        self.gl.draw(args.viewport(),|c, gl| {
            clear(BLACK, gl);
            player.render(&c, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
       self.player.update(args.dt);
    }

    fn input(&mut self, button: &Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::W => self.player.start_move(Position { x: 0.0, y: -1.0 }),
                    Key::S => self.player.start_move(Position { x: 0.0, y: 1.0 }),
                    Key::A => self.player.start_move(Position { x: -1.0, y: 0.0 }),
                    Key::D => self.player.start_move(Position { x: 1.0, y: 0.0 }),
                    _ => (),
                }
            }
        } else {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::W => self.player.stop_move(),
                    Key::S => self.player.stop_move(),
                    Key::A => self.player.stop_move(),
                    Key::D => self.player.stop_move(),
                    _ => (),
                }
            }
        }
    }

    fn mouse_position_update(&mut self, x: f64, y: f64) {
        let mut dx: f64 = x - self.player.position.x;
        let mut dy: f64 = y - self.player.position.y;
        let mut denom: f64 = (dx * dx) + (dy * dy);
        denom = denom.sqrt();
        dx /= denom;
        dy /= denom;
        let rad = dy.atan2(dx);
        self.player.angle = rad;
    }

}

// entry point to game

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("astroids", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: Player {
            position: Position { x: 100.0, y: 100.0 },
            velocity: Position { x: 0.0, y: 0.0 },
            angle: 0.0,
            size: 40.0,
        }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(d) = e.press_args() {
            app.input(&d, true);
        }

        if let Some(d) = e.release_args() {
            app.input(&d, false);
        }

        e.mouse_cursor(|x, y| {
            app.mouse_position_update(x, y);
        });
    }
}