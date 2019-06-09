use super::position::Position;

pub struct Bullet {
    pub position: Position,
    pub velocity: Position,
    pub angle: f64,
    pub size: f64,
    pub alive: bool,
}

impl Bullet {
    pub fn new(x: f64, y: f64, angle: f64, velocity: Position) -> Bullet {
        let mut rng = rand::thread_rng();
        let bullet: Bullet = Bullet {
            position: Position { x: x, y: y },
            velocity: Position { x: velocity.x, y: velocity.y },
            angle: angle,
            size: 20.0,
            alive: true,
        };

        bullet
    }
}

impl PartialEq for Bullet {
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