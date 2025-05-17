use crate::game::point::Point;

#[derive(Debug, PartialEq)]
pub struct CheckPoint {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub r2: f64,
}

impl CheckPoint {
    pub fn from_i32(x: i32, y: i32) -> Self {
        CheckPoint {
            x: x as f64,
            y: y as f64,
            r: 600.0,
            r2: 360000.0,
        }
    }

    pub fn from_f64(x: f64, y: f64) -> Self {
        CheckPoint {
            x,
            y,
            r: 600.0,
            r2: 360000.0,
        }
    }

    pub fn clone_checkpoint(&self) -> Self {
        CheckPoint {
            x: self.x,
            y: self.y,
            r: self.r,
            r2: self.r2,
        }
    }
}

// Implémentation des méthodes héritées de Point
impl CheckPoint {
    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_sq(other).sqrt()
    }

    pub fn distance_sq(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn closest(&self, a: &Point, b: &Point) -> Point {
        // On réutilise la logique de la méthode de Point
        let point = Point {
            x: self.x,
            y: self.y,
        };
        point.closest(a, b)
    }

    pub fn norm_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
}

impl PartialEq<Point> for CheckPoint {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod checkpoint_tests {
    use super::*;

    #[test]
    fn test_checkpoint_creation() {
        let cp = CheckPoint::from_i32(100, 200);
        assert_eq!(cp.x, 100.0);
        assert_eq!(cp.y, 200.0);
        assert_eq!(cp.r, 600.0);
        assert_eq!(cp.r2, 360000.0);
    }

    #[test]
    fn test_checkpoint_distance() {
        let cp = CheckPoint::from_i32(0, 0);
        let p = Point::from_i32(3, 4);
        assert_eq!(cp.distance(&p), 5.0);
    }

    #[test]
    fn test_checkpoint_closest() {
        let cp = CheckPoint::from_i32(3, 2);
        let a = Point::from_i32(0, 0);
        let b = Point::from_i32(6, 0);

        // The closest point from cp to line a-b should be at (3, 0)
        let closest = cp.closest(&a, &b);
        assert_eq!(closest.x, 3.0);
        assert_eq!(closest.y, 0.0);
    }
}
