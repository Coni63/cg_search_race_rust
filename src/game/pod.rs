use std::f64::consts::PI;

use crate::game::action::Action;
use crate::game::checkpoint::CheckPoint;
use crate::game::point::Point;

#[derive(Debug, Clone)]
pub struct Pod {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub angle: f64,
    pub next_checkpoint_id: usize,
    pub r: i32,
}

impl Pod {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, angle: f64, next_checkpoint_id: usize) -> Self {
        Pod {
            x,
            y,
            vx,
            vy,
            angle,
            next_checkpoint_id,
            r: 100,
        }
    }

    pub fn speed(&self) -> f64 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }

    pub fn clone_pod(&self) -> Pod {
        Pod {
            x: self.x,
            y: self.y,
            vx: self.vx,
            vy: self.vy,
            angle: self.angle,
            next_checkpoint_id: self.next_checkpoint_id,
            r: self.r,
        }
    }

    pub fn apply_moves(&mut self, actions: &[Action], checkpoints: &[CheckPoint]) -> i32 {
        let mut cross = 0;
        for action in actions {
            let data = self.apply_move(action, checkpoints);
            if data >= 0.0 {
                cross += 1;
            }
        }

        cross
    }

    pub fn apply_move(&mut self, action: &Action, checkpoints: &[CheckPoint]) -> f64 {
        self._rotate(action.angle as f64);
        self._boost(action.thrust as f64);
        let (cross, data) = self._check_cross_checkpoint(checkpoints);
        self._move();
        self._end();
        data
    }

    pub fn output(&self, action: &Action) -> (f64, f64, i32) {
        let mut next_angle: f64 = self.angle + action.angle as f64;

        if next_angle >= 360.0 {
            next_angle -= 360.0;
        } else if next_angle < 0.0 {
            next_angle += 360.0;
        }

        // On cherche un point pour correspondre à l'angle qu'on veut
        // On multiplie par 10000.0 pour éviter les arrondis
        let next_angle = next_angle * PI / 180.0;
        let px = self.x as f64 + next_angle.cos() * 100000.0;
        let py = self.y as f64 + next_angle.sin() * 100000.0;

        (px, py, action.thrust)
    }

    pub fn describe(&self) {
        eprintln!("");
        eprintln!("Pod Position       : ({}, {})", self.x, self.y);
        eprintln!("Pod Speed          : ({}, {})", self.vx, self.vy);
        eprintln!("Pod Angle          : {}", self.angle);
        eprintln!("Pod NextCheckPoint : {}", self.next_checkpoint_id);
    }

    pub fn get_angle(&self, p: &Point) -> f64 {
        // Returns the angle between the pod and point p relative to the X axis
        let d = self.distance(p);
        let dx = (p.x - self.x) as f64 / d;
        let dy = (p.y - self.y) as f64 / d;

        let a = dx.acos().to_degrees();

        // If the point we want is above us, we need to adjust the angle to make it correct.
        if dy < 0.0 { 360.0 - a } else { a }
    }

    pub fn diff_angle(&self, p: &Point) -> f64 {
        let a = self.get_angle(p);

        // To determine the closest direction, we simply look in both directions and keep the smaller one.
        let right = if self.angle <= a {
            a - self.angle
        } else {
            360.0 - self.angle + a
        };
        let left = if self.angle >= a {
            self.angle - a
        } else {
            self.angle + 360.0 - a
        };

        if right < left {
            right
        } else {
            // We return a negative angle if we need to turn left
            -left
        }
    }

    fn _rotate(&mut self, angle: f64) {
        // rotate the pod by angle degrees (positive = clockwise)

        // We can't turn more than 18 degrees in one turn
        self.angle += angle.max(-18.0).min(18.0);

        // The % operator is slow. If we can avoid it, it's better.
        if self.angle >= 360.0 {
            self.angle -= 360.0;
        } else if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    fn _boost(&mut self, thrust: f64) {
        // Conversion of the angle to radians
        let ra = self.angle * PI / 180.0;

        // Trigonometry
        self.vx += ra.cos() * thrust;
        self.vy += ra.sin() * thrust;
    }

    fn _check_cross_checkpoint(&mut self, checkpoints: &[CheckPoint]) -> (i32, f64) {
        let chkpt_pos = &checkpoints[self.next_checkpoint_id];
        let t = self._has_collision(chkpt_pos);
        if t != -1.0 {
            self.next_checkpoint_id += 1;
            if self.next_checkpoint_id >= checkpoints.len() {
                self.next_checkpoint_id = 0;
            }
            return (1, t);
        }

        (0, -1.0)
    }

    fn _has_collision(&self, chkpt_pos: &CheckPoint) -> f64 {
        // Approach used : https://www.youtube.com/watch?v=23kTf-36Fcw
        let curr_pos = Point::from_f64(self.x, self.y);
        let next_pos = Point::from_f64((self.x + self.vx), (self.y + self.vy));

        // si on est a l'arret, pas besoin de verifier
        if curr_pos == next_pos {
            return -1.0;
        }

        // On cherche le point le plus proche de u (qui est donc en (0,0)) sur la droite décrite par notre vecteur de vitesse
        let p = chkpt_pos.closest(&curr_pos, &next_pos);

        // Distance au carré entre u et le point le plus proche sur la droite décrite par notre vecteur de vitesse
        let b_sq = chkpt_pos.distance_sq(&p);

        // Si la distance entre u et cette droite est inférieur à la somme des rayons, alors il y a possibilité de collision
        if b_sq > chkpt_pos.r2 {
            return -1.0;
        }

        // produit scalaire du centre du checkpoint avec la vitesse
        // s'il est negatif c'est que le pt est en arriere de la trajectoire
        let d = Point::from_f64(p.x - curr_pos.x, p.y - curr_pos.y);
        let s = d.x as f64 * self.vx + d.y as f64 * self.vy;
        if s < 0.0 {
            return -1.0;
        }

        let a_sq = curr_pos.distance_sq(&p);
        if a_sq < 0.0 {
            return -1.0;
        }

        let f = (chkpt_pos.r2 - b_sq).sqrt();
        let t = (a_sq.sqrt() - f) / (self.vx * self.vx + self.vy * self.vy).sqrt();

        if t < 0.0 || t > 1.0 {
            return -1.0;
        }

        t
    }

    fn _move(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }

    fn _end(&mut self) {
        self.x = self.x.trunc();
        self.y = self.y.trunc();
        self.vx = (self.vx * 0.85).trunc();
        self.vy = (self.vy * 0.85).trunc();
        self.angle = self.angle.round();
    }
}

// Implémentation des méthodes héritées de Point
impl Pod {
    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_sq(other).sqrt()
    }

    pub fn distance_sq(&self, other: &Point) -> f64 {
        let dx = self.x as f64 - other.x;
        let dy = self.y as f64 - other.y;
        dx * dx + dy * dy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint(x: i32, y: i32) -> CheckPoint {
        CheckPoint::from_i32(x, y)
    }

    fn pod(x: i32, y: i32, r: i32, vx: i32, vy: i32, angle: i32, next_id: usize) -> Pod {
        Pod::new(
            x as f64,
            y as f64,
            vx as f64,
            vy as f64,
            angle as f64,
            next_id,
        )
    }

    #[test]
    fn test_apply_moves() {
        let mut pod = pod(0, 0, 0, 0, 0, 0, 0);
        let checkpoints = vec![checkpoint(0, 1000)];
        let moves = vec![
            Action {
                thrust: 200,
                angle: 0
            };
            10
        ];

        assert_eq!(pod.vx, 0.0);
        assert_eq!(pod.vy, 0.0);

        pod.apply_moves(&moves, &checkpoints);

        assert_eq!(pod.vx, 907.0);
        assert_eq!(pod.vy, 0.0);
    }

    #[test]
    fn test_vmax_right() {
        let mut pod = pod(0, 0, 0, 0, 0, 0, 0);
        let checkpoints = vec![checkpoint(0, 1000)];
        let speeds = vec![
            170, 314, 436, 540, 629, 704, 768, 822, 868, 907, 940, 969, 993, 1014, 1031, 1046, 1059,
        ];

        for &speed in &speeds {
            let mv = Action {
                thrust: 200,
                angle: 0,
            };
            pod.apply_move(&mv, &checkpoints);
            assert_eq!(pod.vx, speed as f64);
            assert_eq!(pod.vy, 0.0);
        }
    }

    #[test]
    fn test_vmax_left() {
        let mut pod = pod(0, 0, 0, 0, 0, 0, 0);
        let checkpoints = vec![checkpoint(0, 1000)];
        let mv = Action {
            thrust: 200,
            angle: 180,
        };

        pod.apply_move(&mv, &checkpoints);

        assert_eq!(pod.angle, 18.0);
        assert_eq!(pod.vx, 161.0);
        assert_eq!(pod.vy, 52.0);
    }

    #[test]
    fn test_friction() {
        let mut pod = pod(0, 0, 0, 150, 0, 0, 0);
        let checkpoints = vec![checkpoint(0, 1000)];
        let mv = Action {
            thrust: 0,
            angle: 180,
        };

        pod.apply_move(&mv, &checkpoints);

        assert_eq!(pod.vx, 127.0);
    }

    #[test]
    fn test_rotation() {
        let mut pod = pod(0, 0, 0, 0, 0, 0, 0);
        let checkpoints = vec![checkpoint(0, 10000)];

        assert_eq!(pod.angle, 0.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 90,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 18.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 30,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 36.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -30,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 18.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -18,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 0.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 15,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 15.0);
    }

    #[test]
    fn test_rotation_2() {
        let mut pod = pod(0, 0, 0, 5, 5, 45, 0);
        let checkpoints = vec![checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -30,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 27.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -30,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 9.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -30,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 351.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 10,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 1.0);

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: -1,
            },
            &checkpoints,
        );
        assert_eq!(pod.angle, 0.0);
    }

    #[test]
    fn test_cross_checkpoint_1() {
        let mut pod = pod(0, 0, 0, 500, 0, 0, 0);
        let checkpoints = vec![checkpoint(350, 600), checkpoint(0, 100000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 700.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_2() {
        let mut pod = pod(0, 0, 0, 500, 0, 0, 0);
        let checkpoints = vec![checkpoint(350, 599), checkpoint(0, 100000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 700.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_3() {
        let mut pod = pod(0, 0, 0, 500, 0, 0, 1);
        let checkpoints = vec![checkpoint(0, 599), checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 700.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_4() {
        let mut pod = pod(0, 0, 0, 500, 0, 0, 0);
        let checkpoints = vec![checkpoint(700, 599), checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 700.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_5() {
        let mut pod = pod(150, 0, 0, 127, 0, 0, 0);
        let checkpoints = vec![checkpoint(800, 0), checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 477.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_6() {
        let mut pod = pod(150, 0, 0, 1100, 0, 0, 0);
        let checkpoints = vec![checkpoint(800, 50), checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 1450.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }

    #[test]
    fn test_cross_checkpoint_7() {
        let mut pod = pod(1450, 0, 0, -1100, 0, 180, 0);
        let checkpoints = vec![checkpoint(800, 50), checkpoint(0, 10000)];

        pod.apply_move(
            &Action {
                thrust: 200,
                angle: 0,
            },
            &checkpoints,
        );
        assert_eq!(pod.x, 150.0);
        assert_eq!(pod.next_checkpoint_id, 1);
    }
}
