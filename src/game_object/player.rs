pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Player {
    pub position: Position,
    pub velocity: Position,
    pub angle: f64,
    pub size: f64,
}

impl Player {
    pub fn start_move(&mut self, force: Position) {
        self.velocity.x += force.x;
        self.velocity.y += force.y;
    }

    pub fn stop_move(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }
}
