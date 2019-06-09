use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod game_object;

use game_object::{ Player, GameObject, Astroid, Position, Bullet };

pub struct App {
    gl: GlGraphics,
    player: Player,
    astroids: Vec<Astroid>,
    bullets: Vec<Bullet>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let player = &self.player;
        let astroids = &mut self.astroids;
        let bullets = &mut self.bullets;

        self.gl.draw(args.viewport(),|c, gl| {
            clear(BLACK, gl);
            player.render(&c, gl);
            for astroid in astroids {
                astroid.render(&c, gl);
            }
            for bullet in bullets {
                bullet.render(&c, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.player.update(args.dt);
        for astroid in &mut self.astroids {
            astroid.update(args.dt);
        }
        for bullet in &mut self.bullets {
            bullet.update(args.dt);
        }
        self.astroids.retain(|astroid| astroid.alive == true);
        self.bullets.retain(|astroid| astroid.alive == true);
        while self.astroids.len() < 5 {
            self.astroids.push(Astroid::new());
        }

        //TODO: quadtree hit detection
        let mut new_astroids: Vec<Astroid> = Vec::new();
        for astroid in &mut self.astroids {
            for bullet in &mut self.bullets {
                if astroid.collides(bullet) {
                    astroid.alive = false;
                    bullet.alive = false;
                    if astroid.size > 10.0 {
                        for _ in 0..4 {
                            new_astroids.push(Astroid::new_sized(astroid.size, astroid.position.x, astroid.position.y));
                        }
                    }
                }
            }
        }

        for astroid in new_astroids {
            self.astroids.push(astroid);
        }
    }

    fn input(&mut self, button: &Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::W => self.player.start_move(Position { x: 0.0, y: -1.0 }),
                    Key::S => self.player.start_move(Position { x: 0.0, y: 1.0 }),
                    Key::A => self.player.start_move(Position { x: -1.0, y: 0.0 }),
                    Key::D => self.player.start_move(Position { x: 1.0, y: 0.0 }),
                    Key::Space => self.bullets.push(Bullet::new(self.player.position.x + self.player.size / 2.0,
                         self.player.position.y + self.player.size / 2.0, 
                         self.player.angle, 
                         Position { x: 100.0 * self.player.angle.cos(), y: 100.0 * self.player.angle.sin() })),
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
        },
        astroids: Vec::new(),
        bullets: Vec::new(),
    };

    app.astroids.push(Astroid::new());

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

        // conditional compiling for detecting mouse
        #[cfg(target_os = "macos")]
        e.mouse_cursor(|[x, y]| {
            app.mouse_position_update(x, y);
        });

        #[cfg(target_os = "windows")]
        e.mouse_cursor(|x, y| {
            app.mouse_position_update(x, y);
        });

        #[cfg(target_os = "linux")]
        e.mouse_cursor(|[x, y]| {
            app.mouse_position_update(x, y);
        });

    }
}
