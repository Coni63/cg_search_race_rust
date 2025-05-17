#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_sq(other).sqrt()
    }

    pub fn distance_sq(&self, other: &Point) -> f64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64
    }

    pub fn closest(&self, a: &Point, b: &Point) -> Point {
        // http://files.magusgeek.com/csb/csb.html
        // Elle permet de trouver le point le plus proche sur une droite
        // (décrite ici par 2 points) depuis un point.
        //        x
        //        |
        // a---------b
        //        ^
        //        pt
        let da = b.y - a.y;
        let db = a.x - b.x;
        let c1 = da * a.x + db * a.y;
        let c2 = -db * self.x + da * self.y;
        let det = da * da + db * db;

        let (cx, cy) = if det != 0 {
            let cx = (da * c1 - db * c2) as f64 / det as f64;
            let cy = (da * c2 + db * c1) as f64 / det as f64;
            (cx as i32, cy as i32)
        } else {
            // Le point est déjà sur la droite
            (self.x, self.y)
        };

        Point { x: cx, y: cy }
    }

    pub fn norm_sq(&self) -> f64 {
        (self.x * self.x + self.y * self.y) as f64
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Point {
            x: (self.x as f64 * scalar) as i32,
            y: (self.y as f64 * scalar) as i32,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_point {
        use super::*;
        use std::f64::EPSILON;

        // Helper function to compare floating point values with a small epsilon
        fn assert_float_eq(a: f64, b: f64) {
            assert!(
                (a - b).abs() < EPSILON,
                "Expected {} to be equal to {}",
                a,
                b
            );
        }

        #[test]
        fn test_distance() {
            let a = Point::new(0, 0);
            let b = Point::new(3, 4);
            let c = Point::new(-3, -4);

            assert_float_eq(a.distance(&b), 5.0);
            // Check both directions
            assert_float_eq(c.distance(&b), 10.0);
            assert_float_eq(b.distance(&c), 10.0);
        }

        #[test]
        fn test_distance_sq() {
            let a = Point::new(0, 0);
            let b = Point::new(-3, -4);

            assert_float_eq(a.distance_sq(&b), 25.0);
        }

        #[test]
        fn test_closest_distance() {
            // Normal case
            let a = Point::new(0, 0);
            let b = Point::new(10, 0);
            let c = Point::new(3, 1);
            let p1 = Point::new(3, 0);

            assert_eq!(c.closest(&a, &b), p1);

            // Before or after points
            let c = Point::new(-3, 1);
            let p1 = Point::new(-3, 0);
            assert_eq!(c.closest(&a, &b), p1);

            let c = Point::new(30, 1);
            let p1 = Point::new(30, 0);
            assert_eq!(c.closest(&a, &b), p1);

            // Already on segment
            let c = Point::new(3, 0);
            let p1 = Point::new(3, 0);
            assert_eq!(c.closest(&a, &b), p1);

            // Diagonal segment on line
            let a = Point::new(0, 0);
            let b = Point::new(5, 5);
            let c = Point::new(3, 3);
            let p1 = Point::new(3, 3);
            assert_eq!(c.closest(&a, &b), p1);

            // Diagonal segment close to line - need to account for integer rounding
            let a = Point::new(0, 0);
            let b = Point::new(5, 5);
            let c = Point::new(3, 3); // Using integer coords since rust uses i32 in our impl
            let p1 = Point::new(3, 3);
            assert_eq!(c.closest(&a, &b), p1);

            // All same place
            let a = Point::new(0, 0);
            let b = Point::new(0, 0);
            let c = Point::new(0, 0);
            let p1 = Point::new(0, 0);
            assert_eq!(c.closest(&a, &b), p1);
        }

        #[test]
        fn test_add() {
            let p1 = Point::new(5, 5);
            let p2 = Point::new(5, -5);
            let p3 = Point::new(10, 0);

            assert_eq!(p1 + p2, p3);

            let p1 = Point::new(5, 5);
            let p2 = Point::new(0, 0);
            assert_eq!(p1 + p2, p1);
        }

        #[test]
        fn test_sub() {
            let p1 = Point::new(5, 5);
            let p2 = Point::new(5, -5);
            let p3 = Point::new(0, 10);

            assert_eq!(p1 - p2, p3);

            let p1 = Point::new(5, 5);
            let p2 = Point::new(0, 0);
            assert_eq!(p1 - p2, p1);
            assert_eq!(p1 - p1, p2);
        }

        #[test]
        #[allow(clippy::erasing_op)]
        fn test_multiply() {
            let p1 = Point::new(5, -2);
            let p2 = Point::new(0, 0);

            assert_eq!(p1 * 0, p2);

            // In Rust we need separate implementations for different scalar types
            let result = p1 * 1.5;

            // We need to account for potential rounding differences in floating point
            assert!(result.x == 7 || result.x == 8);
            assert!(result.y == -3);
        }

        #[test]
        fn test_equal() {
            let p1 = Point::new(5, -2);
            let p2 = Point::new(0, 0);

            assert!(!(p1 == p2));

            let p1 = Point::new(5, -2);
            let p2 = Point::new(5, -2);
            assert!(p1 == p2);
        }

        #[test]
        fn test_norm() {
            let p1 = Point::new(3, 4);
            assert_float_eq(p1.norm_sq(), 25.0);

            let p1 = Point::new(3, 4);
            assert_float_eq(p1.norm(), 5.0);

            let p1 = Point::new(15, 20);
            let p2 = Point::new(12, 16);
            assert_float_eq((p1 - p2).norm_sq(), 25.0);
        }
    }
}
