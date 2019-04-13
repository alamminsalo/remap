#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    // status which tells if moving is in progress
    pub status: Status,
    // origin of movement
    origin: (i32, i32),
    // position of movement
    position: (i32, i32),
}

impl State {
    // begins movement, storing origin point
    pub fn begin(&mut self, xy: (i32, i32)) {
        self.status = Status::Active;
        self.origin = xy.clone();
        self.position = xy;
    }
    // ends state. returns position
    pub fn end(&mut self) -> (i32, i32) {
        self.status = Status::Inactive;
        self.offset()
    }
    // applies movement
    pub fn set_position(&mut self, xy: (i32, i32)) {
        self.position = xy;
    }
    pub fn is_moving(&self) -> bool {
        self.status == Status::Active
    }
    pub fn offset(&self) -> (i32, i32) {
        (
            self.position.0 - self.origin.0,
            self.position.1 - self.origin.1,
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Inactive,
    Active,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}
