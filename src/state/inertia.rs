#[derive(Default, PartialEq, Clone, Copy)]
pub struct State {
    // status which tells if moving is in progress
    status: Status,
    // velocity vector, px per tick
    velocity: (f64, f64),
    // slow factor
    slow_factor: f64,
}

impl State {
    pub fn begin(velocity: (f64, f64)) -> Self {
        // console!(log, "apply inertia", &velocity.0, &velocity.1);
        Self {
            status: Status::InProgress,
            slow_factor: 0.92,
            velocity,
        }
    }
    pub fn tick(&mut self, dt: f64) -> (f64, f64) {
        // slow movement
        let slow = self.slow_factor * (1.0 - dt);
        self.velocity.0 *= slow;
        self.velocity.1 *= slow;

        if self.velocity.0.abs() < 0.5 && self.velocity.1.abs() < 0.5 {
            self.velocity = (0.0, 0.0);
            self.status = Status::Ended;
        }

        self.velocity
    }
    pub fn status(&self) -> Status {
        self.status
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Ended,
    InProgress,
}

impl Default for Status {
    fn default() -> Self {
        Status::InProgress
    }
}
