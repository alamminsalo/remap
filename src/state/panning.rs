use stdweb::web::Date;

/// State struct for map panning changes.
#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    /// status which tells if moving is in progress
    pub status: Status,
    /// origin of movement
    origin: (f64, f64),
    /// position of movement
    position: (f64, f64),
    /// velocity of movement
    velocity: (f64, f64),
    /// time, used for calculating velocity between last two movements
    time: f64,
}

impl State {
    /// begins movement, storing origin point
    pub fn begin(&mut self, xy: (f64, f64)) {
        self.status = Status::Panning;
        self.origin = xy.clone();
        self.position = xy;
        self.time = Date::now();
        self.velocity = (0.0, 0.0);
    }
    /// Releases panning ctrl.
    /// Returns velocity between last two movements,
    /// which can be used to start inertia state.
    pub fn release(&mut self) -> (f64, f64) {
        self.status = Status::Free;
        self.velocity
    }
    /// Ends state. Returns position
    pub fn end(&mut self) -> (f64, f64) {
        self.status = Status::Idle;
        self.offset()
    }
    /// Sets new movement position
    pub fn set_position(&mut self, xy: (f64, f64)) {
        self.position = xy;

        // buildup velocity
        let now = Date::now();
        let dt = (now - self.time).max(1.0);
        let o = self.offset();
        self.velocity = (o.0 as f64 / dt, o.1 as f64 / dt);
        console!(
            log,
            "dt",
            &dt,
            "velocity",
            &self.velocity.0,
            &self.velocity.1
        );
        self.time = now;
    }
    /// Adds relative positioning, this does not update current velocity.
    /// Is used by inertia to update position each tick.
    pub fn add_relative(&mut self, xy: (f64, f64)) {
        self.position.0 += xy.0;
        self.position.1 += xy.1;
    }

    /// Returns current status
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
