use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Rotator {
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Rotator { pitch, yaw, roll }
    }
}

impl Display for Rotator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pitch: {} yaw: {} roll: {}",
            self.pitch, self.yaw, self.roll
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quat { x, y, z, w }
    }
}

impl Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {} w: {}", self.x, self.y, self.z, self.w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub ticks: u64,
}

impl DateTime {
    pub fn new(ticks: u64) -> Self {
        DateTime { ticks }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ticks: {}", self.ticks)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntPoint {
    pub x: i32,
    pub y: i32,
}

impl IntPoint {
    pub fn new(x: i32, y: i32) -> Self {
        IntPoint { x, y }
    }
}

impl Display for IntPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}
