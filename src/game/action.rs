#[derive(Debug)]
pub struct Action {
    pub thrust: i32,
    pub angle: i32,
}

impl Action {
    pub fn new(thrust: i32, angle: i32) -> Self {
        Self { thrust, angle }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.thrust, self.angle)
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let thrust = parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        let angle = parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        Action::new(thrust, angle)
    }
}
