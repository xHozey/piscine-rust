#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        Self {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            time: 0.,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.;

        self.actual_velocity = Object {
            x: self.init_velocity.x,
            y: ((self.init_velocity.y - 9.8 * self.time) * 100.).round() / 100.,
        };
        let x = self.init_position.x + self.init_velocity.x * self.time;
        let y = self.init_position.y + self.init_velocity.y * self.time
            - 1. / 2. * 9.8 * self.time * self.time;
        self.actual_position = Object {
            x: x,
            y: (y * 100.).round() / 100.,
        };
        if self.actual_position.y <= 0. {
            return None;
        }
        Some(Self {
            init_position: self.init_position.clone(),
            init_velocity: self.init_velocity.clone(),
            actual_position: self.actual_position.clone(),
            actual_velocity: self.actual_velocity.clone(),
            time: self.time,
        })
    }
}
