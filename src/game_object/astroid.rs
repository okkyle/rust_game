use super::position::Position;

use rand::Rng;

pub struct Astroid {
    pub position: Position,
    pub velocity: Position,
    pub angle: f64,
    pub size: f64,
}

impl Astroid {
    pub fn new() -> Astroid {
        let mut rng = rand::thread_rng();
        let astroid: Astroid = Astroid {
            position: Position { x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0) },
            velocity: Position { x: rng.gen_range(0.0, 10.0), y: rng.gen_range(0.0, 10.0) },
            angle: rng.gen_range(0.0, 6.283),
            size: rng.gen_range(1.0, 50.0),

        };

        astroid
    }
}
