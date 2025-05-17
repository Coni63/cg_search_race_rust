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

#[derive(Debug)]
pub struct GameManager {
    pub checkpoints: Vec<CheckPoint>,
    pub pod: Pod,
    pub done: bool,
    pub turn: usize,
    pub max_turn: usize,
}

impl GameManager {
    pub fn clone_manager(&self) -> GameManager {
        GameManager {
            checkpoints: self.checkpoints.clone(),
            pod: self.pod.clone_pod(),
            done: self.done,
            turn: self.turn,
            max_turn: self.max_turn,
        }
    }

    pub fn apply_action(&mut self, action: &Action) -> (&Pod, bool, f64) {
        let t = self.pod.apply_move(action, &self.checkpoints);
        self.turn += 1;

        // game is done when the target is the last checkpoint which is a fictive one aligned with the 2 last ones
        self.done = (self.pod.next_checkpoint_id == self.checkpoints.len() - 1)
            || (self.turn == self.max_turn);

        (&self.pod, self.done, t)
    }

    pub fn apply_actions(&mut self, actions: &[Action]) -> (&Pod, bool, Option<f64>) {
        for action in actions {
            let (_, done, _) = self.apply_action(action);
            if done {
                break;
            }
        }

        (&self.pod, self.done, None)
    }

    pub fn reset(&mut self) {
        let starting_position = &self.checkpoints[self.checkpoints.len() - 2];
        self.pod = Pod::new(starting_position.x, starting_position.y, 0.0, 0.0, 0.0, 0);

        let checkpoint_point = Point::from_f64(self.checkpoints[0].x, self.checkpoints[0].y);
        let angle = self.pod.get_angle(&checkpoint_point);
        self.pod.angle = angle.round();

        self.done = false;
        self.turn = 0;
    }

    pub fn from_checkpoints(checkpoints: Vec<CheckPoint>) -> Self {
        let mut game = GameManager {
            checkpoints,
            pod: Pod::new(0.0, 0.0, 0.0, 0.0, 0.0, 0),
            done: false,
            turn: 0,
            max_turn: 600,
        };
        game.reset();
        game
    }
}

impl<P: AsRef<Path>> From<P> for GameManager {
    fn from(testcase: P) -> Self {
        let mut file = File::open(testcase).expect("Failed to open testcase file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read testcase file");

        let mut checkpoints: Vec<CheckPoint> = Vec::new();

        let data: Option<TestData> = serde_json::from_str(&content).expect("Failed to parse JSON");
        if let Some(data) = &data {
            let mut all_pts = Vec::new();

            for s in data.test_in.split(';') {
                let coords: Vec<i32> = s.split(' ').filter_map(|x| x.parse::<i32>().ok()).collect();
                if coords.len() >= 2 {
                    all_pts.push(CheckPoint::from_i32(coords[0], coords[1]));
                }
            }

            // Rotated checkpoints (equivalent to all_pts[1:] + all_pts[:1] in Python)
            let mut rotated_chkpt = Vec::new();
            if !all_pts.is_empty() {
                rotated_chkpt.extend_from_slice(&all_pts[1..]);
                rotated_chkpt.push(all_pts[0]);
            }

            // Multiplier par 3 (equivalent to rotated_chkpt * 3 in Python)
            checkpoints = rotated_chkpt.clone();
            checkpoints.extend(rotated_chkpt.clone());
            checkpoints.extend(rotated_chkpt);

            let n_minus2 = &checkpoints[checkpoints.len() - 2];
            let n_minus1 = &checkpoints[checkpoints.len() - 1];

            // Conversion en Point pour utiliser les opérations
            let n_minus2_point = Point::from_f64(n_minus2.x, n_minus2.y);
            let n_minus1_point = Point::from_f64(n_minus1.x, n_minus1.y);

            let dist = n_minus2.distance(&n_minus1_point);
            let factor = 50_000.0 / dist;

            // Calcul du dernier point
            // last_pt = n_minus1 * (factor+1) - n_minus2 * factor
            let n_minus1_scaled = n_minus1_point * (factor + 1.0);
            let n_minus2_scaled = n_minus2_point * factor;
            let last_pt = Point::from_f64(
                (n_minus1_scaled.x - n_minus2_scaled.x).trunc(),
                (n_minus1_scaled.y - n_minus2_scaled.y).trunc(),
            );

            checkpoints.push(CheckPoint::from_f64(last_pt.x, last_pt.y));
        }

        GameManager::from_checkpoints(checkpoints)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Action, CheckPoint, GameManager, Pod};

    #[test]
    fn test_start_position() {
        let game = GameManager::from("testcases/test1.json");

        assert_eq!(game.pod.x, 10353.0);
        assert_eq!(game.pod.y, 1986.0);
        assert_eq!(game.pod.angle.round(), 161.0);
        assert_eq!(game.pod.vx, 0.0);
        assert_eq!(game.pod.vy, 0.0);
        assert_eq!(game.pod.next_checkpoint_id, 0);
    }

    #[test]
    fn test_checkpoint_list() {
        let game = GameManager::from("testcases/test1.json");

        let expected = vec![
            CheckPoint::from_i32(2757, 4659),
            CheckPoint::from_i32(3358, 2838),
            CheckPoint::from_i32(10353, 1986),
            CheckPoint::from_i32(2757, 4659),
            CheckPoint::from_i32(3358, 2838),
            CheckPoint::from_i32(10353, 1986),
            CheckPoint::from_i32(2757, 4659),
            CheckPoint::from_i32(3358, 2838),
            CheckPoint::from_i32(10353, 1986),
            CheckPoint::from_i32(59986, -4059),
        ];

        assert_eq!(game.checkpoints, expected);
    }

    #[test]
    fn test_apply_move() {
        let checkpoints = vec![
            CheckPoint::from_i32(800, 0),
            CheckPoint::from_i32(2200, 0),
            CheckPoint::from_i32(3600, 0),
        ];
        let mut game = GameManager::from_checkpoints(checkpoints);

        assert_eq!(game.turn, 0);

        let (mut _pod, mut done, mut _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 1);
        assert!(!done);

        for _ in 0..598 {
            (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        }
        assert_eq!(game.turn, 599);
        assert!(!done);

        (_pod, done, _turn) = game.apply_action(&Action::new(1, 0));
        assert_eq!(game.turn, 600);
        assert!(done);
    }

    #[test]
    fn test_apply_moves() {
        let checkpoints = vec![
            CheckPoint::from_i32(800, 0),
            CheckPoint::from_i32(2200, 0),
            CheckPoint::from_i32(3600, 0),
        ];
        let mut game = GameManager::from_checkpoints(checkpoints);

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
        let mut game = GameManager::from("testcases/test1.json");

        // Assert initial values
        assert_eq!(game.pod.x, 10353.0);
        assert_eq!(game.pod.y, 1986.0);
        assert_eq!(game.pod.angle.round(), 161.0);

        // Apply the action
        game.apply_action(&Action::new(200, 0));

        // Assert values after action
        assert_eq!(game.pod.x, 10163.0);
        assert_eq!(game.pod.y, 2051.0);

        // Now clone the game (which requires another mutable borrow)
        let game2 = game.clone_manager();

        // Make assertions using our previously saved values
        assert_eq!(game.pod.x, game2.pod.x);
        assert_eq!(game.pod.y, game2.pod.y);
        assert!(!std::ptr::eq(&game, &game2));
    }

    #[test]
    fn test_simulation_1() {
        let mut game = GameManager::from("testcases/test13.json");

        assert_eq!(game.pod.x, 13332.0);
        assert_eq!(game.pod.y, 4114.0);
        assert_eq!(game.pod.angle.round(), 154.0);
        assert_eq!(game.pod.vx, 0.0);
        assert_eq!(game.pod.vy, 0.0);
        assert_eq!(game.pod.next_checkpoint_id, 0);

        let command = "200,-14;200,-1;200,14;200,7;200,-12;176,0;200,18;116,18;100,18;0,18;0,18;15,18;146,18;200,18;200,18;195,18;200,9;200,6;200,15;200,15;200,11;200,11;161,-3;191,18;98,18;18,18;0,18;20,18;87,11;200,18;197,18;200,18;200,18;200,14;151,-7;166,9;200,-8;200,10;197,-1;200,7;200,-7;200,-18;200,-18;123,-18;7,-18;0,-18;0,-18;44,-18;58,-18;158,-18;176,-18;200,-18;200,-18;200,-17;200,-15;200,-2;200,0;200,9;198,-9;184,17;184,8;173,18;157,18;120,12;0,18;178,18;131,16;200,15;200,18;200,15;200,11;200,-9;200,9;200,4;194,10;200,14;171,16;200,15;16,18;53,18;0,18;200,18;200,15;200,18;194,18;200,12;200,14;176,8;143,15;188,18;192,-18;200,1;157,-18;11,-18;13,-18;0,-18;0,-18;3,-18;200,-18;200,-18;200,-16;200,-18;200,-3;200,-11;200,-18;200,6;200,10;200,2;200,-4;191,0;200,-18;200,-18;195,-18;37,-16;27,-18;0,-18;22,-18;5,-18;103,-18;200,-18;197,-9;200,-8;200,-6;200,-18;200,2;200,-5;200,6;194,11;200,18;200,8;200,18;149,9;91,18;64,14;114,13;160,18;200,18;200,18;200,11;190,-13;200,18;200,11;200,3;187,-4;200,18;198,18;92,18;90,18;12,18;174,18;199,18;200,18;200,14;200,18;200,14;200,3;200,18;199,1;200,15;200,5;193,15;92,18;0,18;10,18;0,18;43,18;200,18;200,18;200,18;200,11;200,0;198,0;200,18;200,-6;200,11;200,-11;200,-9;185,-10;198,-8;176,-12;37,-18;9,-18;0,-18;0,-18;48,-18;145,-18;200,-18;200,-18;200,-18;200,-18;200,-18;200,-18;200,18;149,-17;199,7;71,-10;179,5;169,18;157,18;139,10;185,16;187,16;176,15;0,18;200,18;200,18;200,0;200,-8";

        let actions: Vec<Action> = command.split(';').map(Action::from).collect();

        for (i, action) in actions.iter().enumerate() {
            let (_, done, t) = game.apply_action(action);
            if done {
                let error_score = i as f64 + t - 207.57;
                assert!(error_score.abs() < 0.1, "Error score: {}", error_score);
                return;
            }
        }

        panic!("Game should have ended before the last action");
    }
}
