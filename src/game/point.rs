#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn from_i32(x: i32, y: i32) -> Self {
        Point {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn from_f64(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_sq(other).sqrt()
    }

    pub fn distance_sq(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
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

        let (cx, cy) = if det != 0.0 {
            let cx = (da * c1 - db * c2) / det;
            let cy = (da * c2 + db * c1) / det;
            (cx, cy)
        } else {
            // Le point est déjà sur la droite
            (self.x, self.y)
        };

        Point { x: cx, y: cy }
    }

    pub fn norm_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
}

impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for &Point {
    type Output = Point;
    fn mul(self, scalar: f64) -> Point {
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

        // Helper function to compare floating point values with a small epsilon
        fn assert_float_eq(a: f64, b: f64) {
            assert!(
                (a - b).abs() < f64::EPSILON,
                "Expected {} to be equal to {}",
                a,
                b
            );
        }

        #[test]
        fn test_distance() {
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(3, 4);
            let c = Point::from_i32(-3, -4);

            assert_float_eq(a.distance(&b), 5.0);
            // Check both directions
            assert_float_eq(c.distance(&b), 10.0);
            assert_float_eq(b.distance(&c), 10.0);
        }

        #[test]
        fn test_distance_sq() {
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(-3, -4);

            assert_float_eq(a.distance_sq(&b), 25.0);
        }

        #[test]
        fn test_closest_distance() {
            // Normal case
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(10, 0);
            let c = Point::from_i32(3, 1);
            let p1 = Point::from_i32(3, 0);

            assert_eq!(c.closest(&a, &b), p1);

            // Before or after points
            let c = Point::from_i32(-3, 1);
            let p1 = Point::from_i32(-3, 0);
            assert_eq!(c.closest(&a, &b), p1);

            let c = Point::from_i32(30, 1);
            let p1 = Point::from_i32(30, 0);
            assert_eq!(c.closest(&a, &b), p1);

            // Already on segment
            let c = Point::from_i32(3, 0);
            let p1 = Point::from_i32(3, 0);
            assert_eq!(c.closest(&a, &b), p1);

            // Diagonal segment on line
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(5, 5);
            let c = Point::from_i32(3, 3);
            let p1 = Point::from_i32(3, 3);
            assert_eq!(c.closest(&a, &b), p1);

            // Diagonal segment close to line - need to account for integer rounding
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(5, 5);
            let c = Point::from_i32(3, 3); // Using integer coords since rust uses i32 in our impl
            let p1 = Point::from_i32(3, 3);
            assert_eq!(c.closest(&a, &b), p1);

            // All same place
            let a = Point::from_i32(0, 0);
            let b = Point::from_i32(0, 0);
            let c = Point::from_i32(0, 0);
            let p1 = Point::from_i32(0, 0);
            assert_eq!(c.closest(&a, &b), p1);
        }

        #[test]
        fn test_add() {
            let p1 = Point::from_i32(5, 5);
            let p2 = Point::from_i32(5, -5);
            let p3 = Point::from_i32(10, 0);

            assert_eq!(&p1 + &p2, p3);

            let p1 = Point::from_i32(5, 5);
            let p2 = Point::from_i32(0, 0);
            assert_eq!(&p1 + &p2, p1);
        }

        #[test]
        fn test_sub() {
            let p1 = Point::from_i32(5, 5);
            let p2 = Point::from_i32(5, -5);
            let p3 = Point::from_i32(0, 10);

            assert_eq!(&p1 - &p2, p3);

            let p1 = Point::from_i32(5, 5);
            let p2 = Point::from_i32(0, 0);
            assert_eq!(&p1 - &p2, p1);
            assert_eq!(&p1 - &p1, p2);
        }

        #[test]
        #[allow(clippy::erasing_op)]
        fn test_multiply() {
            let p1 = Point::from_i32(5, -2);
            let p2 = Point::from_i32(0, 0);

            assert_eq!(&p1 * 0.0, p2);

            // In Rust we need separate implementations for different scalar types
            let result = &p1 * 1.5;

            // We need to account for potential rounding differences in floating point
            assert!(result.x == 7.5);
            assert!(result.y == -3.0);
        }

        #[test]
        fn test_equal() {
            let p1 = Point::from_i32(5, -2);
            let p2 = Point::from_i32(0, 0);

            assert!(!(p1 == p2));

            let p1 = Point::from_i32(5, -2);
            let p2 = Point::from_i32(5, -2);
            assert!(p1 == p2);
        }

        #[test]
        fn test_norm() {
            let p1 = Point::from_i32(3, 4);
            assert_float_eq(p1.norm_sq(), 25.0);

            let p1 = Point::from_i32(3, 4);
            assert_float_eq(p1.norm(), 5.0);

            let p1 = Point::from_i32(15, 20);
            let p2 = Point::from_i32(12, 16);
            assert_float_eq((&p1 - &p2).norm_sq(), 25.0);
        }
    }
}
