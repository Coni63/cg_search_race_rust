#[derive(Debug, Clone)]
pub struct Action {
    pub thrust: u8,
    pub angle: i16,
}

impl Action {
    pub fn new(thrust: u8, angle: i16) -> Self {
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
        let thrust = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        let angle = parts.next().unwrap_or("0").parse::<i16>().unwrap_or(0);
        Action::new(thrust, angle)
    }
}
