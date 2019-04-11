#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    // status which tells if moving is in progress
    pub status: Status,
    // origin of movement
    pub origin: (i32, i32),
    // position of movement
    pub position: (i32, i32),
}

impl State {
    // begins movement, storing origin point
    pub fn begin(&mut self, point: (i32, i32)) {
        self.status = Status::Moving;
        self.origin = point;
        self.position = point;
    }
    // ends state. returns offset from origin and point
    pub fn end(&mut self, point: (i32, i32)) -> (i32, i32) {
        self.status = Status::NotMoving;
        self.position = point;
        self.offset()
    }
    // applies movement
    pub fn set_position(&mut self, point: (i32, i32)) -> (i32, i32) {
        self.position = point;
        self.offset()
    }
    // returns offset from origin and point
    pub fn offset(&self) -> (i32, i32) {
        (
            self.position.0 - self.origin.0,
            self.position.1 - self.origin.1,
        )
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
