use stdweb::web::Date;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    // status which tells if moving is in progress
    pub status: Status,
    // origin of movement
    origin: (f64, f64),
    // position of movement
    position: (f64, f64),
    // velocity of movement
    velocity: (f64, f64),
    // time
    time: f64,
}

impl State {
    // begins movement, storing origin point
    pub fn begin(&mut self, xy: (f64, f64)) {
        self.status = Status::Panning;
        self.origin = xy.clone();
        self.position = xy;
        self.time = Date::now();
        self.velocity = (0.0, 0.0);
    }
    // frees panning ctrl. returns velocity
    pub fn release(&mut self) -> (f64, f64) {
        self.status = Status::Free;
        self.velocity
    }
    // ends state. returns position
    pub fn end(&mut self) -> (f64, f64) {
        self.status = Status::Idle;
        self.offset()
    }
    // applies movement
    pub fn set_position(&mut self, xy: (f64, f64)) {
        self.position = xy;

        // buildup velocity
        let dt = Date::now() - self.time;
        let o = self.offset();
        self.velocity = (o.0 as f64 / dt, o.1 as f64 / dt);
        self.time += dt;
    }
    pub fn add_relative(&mut self, xy: (f64, f64)) {
        self.position.0 += xy.0;
        self.position.1 += xy.1;
    }
    pub fn status(&self) -> Status {
        self.status
    }
    pub fn offset(&self) -> (f64, f64) {
        (
            self.position.0 - self.origin.0,
            self.position.1 - self.origin.1,
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Idle,
    Panning,
    Free,
}

impl Default for Status {
    fn default() -> Self {
        Status::Idle
    }
}
