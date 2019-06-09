use super::position::Position;

use rand::Rng;

pub struct Astroid {
    pub position: Position,
    pub velocity: Position,
    pub angle: f64,
    pub size: f64,
    pub alive: bool,
}

impl Astroid {
    pub fn new() -> Astroid {
        let mut rng = rand::thread_rng();
        let astroid: Astroid = Astroid {
            position: Position { x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0) },
            velocity: Position { x: rng.gen_range(0.0, 10.0), y: rng.gen_range(0.0, 10.0) },
            angle: rng.gen_range(0.0, 6.283),
            size: rng.gen_range(40.0, 200.0),
            alive: true,
        };

        astroid
    }

    pub fn new_sized(size: f64, x: f64, y: f64) -> Astroid {
        let mut rng = rand::thread_rng();
        let astroid: Astroid = Astroid {
            position: Position { x: x + rng.gen_range(-20.0, 20.0), y: y + rng.gen_range(-20.0, 20.0) },
            velocity: Position { x: rng.gen_range(0.0, 10.0), y: rng.gen_range(0.0, 10.0) },
            angle: rng.gen_range(0.0, 6.283),
            size: rng.gen_range(size / 10.0, size),
            alive: true,
        };

        astroid
    }
}

impl PartialEq for Astroid {
    fn eq(&self, other: &Self) -> bool {
        if self.position.x == other.position.x && self.position.y == other.position.y {
            if self.angle == other.angle && self.size == other.size {
                if self.alive == other.alive {
                    return true;
                }
            }
        }
        false
    }
}