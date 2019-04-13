#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    // status which tells if moving is in progress
    pub status: Status,
    // position of movement
    pub position: (i32, i32),
}

impl State {
    // begins movement, storing origin point
    pub fn begin(&mut self) {
        self.status = Status::Moving;
        self.position = (0, 0);
    }
    // ends state. returns position
    pub fn end(&mut self) -> (i32, i32) {
        self.status = Status::NotMoving;
        self.position
    }
    // applies movement
    pub fn add_movement(&mut self, movement: (i32, i32)) {
        self.position = (self.position.0 + movement.0, self.position.1 + movement.1);
    }
    pub fn is_moving(&self) -> bool {
        self.status == Status::Moving
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    NotMoving,
    Moving,
}

impl Default for Status {
    fn default() -> Self {
        Status::NotMoving
    }
}
