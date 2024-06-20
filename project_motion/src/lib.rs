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
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
    fn round_to_one_decimal_place(value: f32) -> f32 {
        (value * 10.0).round() / 10.0
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let gravity = -9.8;
        let new_time = self.time + 1.0;
        let new_x = self.init_position.x + self.init_velocity.x * new_time;
        let new_y = self.init_position.y + self.init_velocity.y * new_time + 0.5 * gravity * new_time * new_time;
        let new_velocity_x = self.init_velocity.x;
        let new_velocity_y = self.init_velocity.y + gravity * new_time;
        self.actual_position = Object {
            x: Self::round_to_one_decimal_place(new_x),
            y: Self::round_to_one_decimal_place(new_y),
        };
        self.actual_velocity = Object {
            x: Self::round_to_one_decimal_place(new_velocity_x),
            y: Self::round_to_one_decimal_place(new_velocity_y),
        };
        self.time = new_time;
        if self.actual_position.y <= 0.0 {
            return None;
        }
        Some(self.clone())
    }
}