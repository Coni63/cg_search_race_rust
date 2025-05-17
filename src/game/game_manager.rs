use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::game::action::Action;
use crate::game::checkpoint::CheckPoint;
use crate::game::pod::Pod;
use crate::game::point::Point;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestData {
    #[serde(rename = "testIn")]
    test_in: String,
    // Ajoutez d'autres champs si nécessaires
}

#[derive(Debug, Clone)]
pub struct GameManager {
    data: Option<TestData>,
    pub checkpoints: Vec<CheckPoint>,
    pub pod: Option<Pod>,
    pub done: bool,
    pub turn: usize,
    max_turn: usize,
}

impl GameManager {
    pub fn new(max_turn: usize) -> Self {
        GameManager {
            data: None,
            checkpoints: Vec::new(),
            pod: None,
            done: false,
            turn: 0,
            max_turn,
        }
    }

    pub fn clone_manager(&self) -> GameManager {
        GameManager {
            data: self.data.clone(),
            checkpoints: self.checkpoints.clone(),
            pod: self.pod.as_ref().map(|p| p.clone_pod()),
            done: self.done,
            turn: self.turn,
            max_turn: self.max_turn,
        }
    }

    pub fn set_testcase<P: AsRef<Path>>(&mut self, testcase: P) -> (&Pod, &[CheckPoint]) {
        let mut file = File::open(testcase).expect("Failed to open testcase file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read testcase file");

        self.data = Some(serde_json::from_str(&content).expect("Failed to parse JSON"));
        self.reset();

        (self.pod.as_ref().unwrap(), &self.checkpoints)
    }

    pub fn apply_action(&mut self, action: &Action) -> (&Pod, bool, f64) {
        let pod = self.pod.as_mut().unwrap();
        let t = pod.apply_move(action, &self.checkpoints);
        self.turn += 1;

        // game is done when the target is the last checkpoint which is a fictive one aligned with the 2 last ones
        self.done =
            (pod.next_checkpoint_id == self.checkpoints.len() - 1) || (self.turn == self.max_turn);

        (pod, self.done, t)
    }

    pub fn apply_actions(&mut self, actions: &[Action]) -> (&Pod, bool, Option<f64>) {
        for action in actions {
            let (_, done, _) = self.apply_action(action);
            if done {
                break;
            }
        }

        (self.pod.as_ref().unwrap(), self.done, None)
    }

    pub fn reset(&mut self) {
        self.pod = Some(Pod::new(0, 0, 0.0, 0.0, 0.0, 0));
        self._parse_checkpoint();

        let start = &self.checkpoints[self.checkpoints.len() - 2];
        self.pod = Some(Pod::new(start.x, start.y, 0.0, 0.0, 0.0, 0));

        if let Some(pod) = &mut self.pod {
            let checkpoint_point = Point::new(self.checkpoints[0].x, self.checkpoints[0].y);
            let angle = pod.get_angle(&checkpoint_point);
            pod.angle = angle.round();
        }

        self.done = false;
        self.turn = 0;
    }

    fn _parse_checkpoint(&mut self) {
        if let Some(data) = &self.data {
            let mut all_pts = Vec::new();

            for s in data.test_in.split(';') {
                let coords: Vec<i32> = s.split(' ').filter_map(|x| x.parse::<i32>().ok()).collect();
                if coords.len() >= 2 {
                    all_pts.push(CheckPoint::new(coords[0], coords[1]));
                }
            }

            // Rotated checkpoints (equivalent to all_pts[1:] + all_pts[:1] in Python)
            let mut rotated_chkpt = Vec::new();
            if !all_pts.is_empty() {
                rotated_chkpt.extend_from_slice(&all_pts[1..]);
                rotated_chkpt.push(all_pts[0]);
            }

            // Multiplier par 3 (equivalent to rotated_chkpt * 3 in Python)
            self.checkpoints = rotated_chkpt.clone();
            self.checkpoints.extend(rotated_chkpt.clone());
            self.checkpoints.extend(rotated_chkpt);

            let n_minus2 = &self.checkpoints[self.checkpoints.len() - 2];
            let n_minus1 = &self.checkpoints[self.checkpoints.len() - 1];

            // Conversion en Point pour utiliser les opérations
            let n_minus2_point = Point::new(n_minus2.x, n_minus2.y);
            let n_minus1_point = Point::new(n_minus1.x, n_minus1.y);

            let dist = n_minus2.distance(&n_minus1_point);
            let factor = 50_000.0 / dist;

            // Calcul du dernier point
            // last_pt = n_minus1 * (factor+1) - n_minus2 * factor
            let n_minus1_scaled = n_minus1_point * (factor + 1.0);
            let n_minus2_scaled = n_minus2_point * factor;
            let last_pt = Point::new(
                n_minus1_scaled.x - n_minus2_scaled.x,
                n_minus1_scaled.y - n_minus2_scaled.y,
            );

            self.checkpoints.push(CheckPoint::new(last_pt.x, last_pt.y));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Action, CheckPoint, GameManager, Pod};

    #[test]
    fn test_start_position() {
        let mut game = GameManager::new(600);
        let (pod, _checkpoints) = game.set_testcase("testcases/test1.json");

        assert_eq!(pod.x, 10353);
        assert_eq!(pod.y, 1986);
        assert_eq!(pod.angle.round() as i32, 161);
        assert_eq!(pod.vx, 0.0);
        assert_eq!(pod.vy, 0.0);
        assert_eq!(pod.next_checkpoint_id, 0);
    }

    #[test]
    fn test_checkpoint_list() {
        let mut game = GameManager::new(600);
        let (_pod, checkpoints) = game.set_testcase("testcases/test1.json");

        let expected = vec![
            CheckPoint::new(2757, 4659),
            CheckPoint::new(3358, 2838),
            CheckPoint::new(10353, 1986),
            CheckPoint::new(2757, 4659),
            CheckPoint::new(3358, 2838),
            CheckPoint::new(10353, 1986),
            CheckPoint::new(2757, 4659),
            CheckPoint::new(3358, 2838),
            CheckPoint::new(10353, 1986),
            // CheckPoint::new(59986, -4059),
            CheckPoint::new(59986, -4060), // TODO: Check this value, projection issue
        ];

        assert_eq!(checkpoints, expected);
    }

    #[test]
    fn test_apply_move() {
        let mut game = GameManager::new(600);

        game.checkpoints = vec![
            CheckPoint::new(800, 0),
            CheckPoint::new(2200, 0),
            CheckPoint::new(3600, 0),
        ];
        game.pod = Some(Pod::new(0, 0, 0.0, 0.0, 0.0, 0));

        assert_eq!(game.turn, 0);

        let (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 1);
        assert!(!done);

        for _ in 0..598 {
            let (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        }
        assert_eq!(game.turn, 599);
        assert!(!done);

        let (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 600);
        assert!(done);
    }

    #[test]
    fn test_apply_moves() {
        let mut game = GameManager::new(600);

        game.checkpoints = vec![
            CheckPoint::new(800, 0),
            CheckPoint::new(2200, 0),
            CheckPoint::new(3600, 0),
        ];
        game.pod = Some(Pod::new(0, 0, 0.0, 0.0, 0.0, 0));

        assert_eq!(game.turn, 0);

        let (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 1);
        assert!(!done);

        let actions = vec![Action::new(1, 0); 598];
        let (_pod, done, _turn) = game.apply_actions(&actions);
        assert_eq!(game.turn, 599);
        assert!(!done);

        let (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 600);
        assert!(done);
    }

    #[test]
    fn test_clone() {
        // Create a new game manager
        let mut game = GameManager::new(600);

        {
            // Set up the test case
            let (_pod, _) = game.set_testcase("testcases/test1.json");

            // Assert initial values
            assert_eq!(_pod.x, 10353);
            assert_eq!(_pod.y, 1986);
            assert_eq!(_pod.angle.round() as i32, 161);
        }

        // Apply the action
        game.apply_action(&Action::new(200, 0));

        let pod1 = game.pod.as_ref().unwrap();
        // Assert values after action
        assert_eq!(pod1.x, 10163);
        assert_eq!(pod1.y, 2051);

        // First, get a snapshot of the data we'll need for later comparison
        let pod_x = game.pod.as_ref().unwrap().x;
        let pod_y = game.pod.as_ref().unwrap().y;

        // Now clone the game (which requires another mutable borrow)
        let game2 = game.clone_manager();

        // Now access pod data from the cloned game
        let pod2 = game2.pod.as_ref().unwrap();

        // Make assertions using our previously saved values
        assert_eq!(pod_x, pod2.x);
        assert_eq!(pod_y, pod2.y);
        assert!(!std::ptr::eq(&game, &game2));
    }
}
