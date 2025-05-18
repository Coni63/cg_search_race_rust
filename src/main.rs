mod game;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

use game::action::Action;
use game::checkpoint::CheckPoint;
use game::pod::Pod;
use game::point::Point;

#[derive(Debug, Serialize, Deserialize)]
struct TestData {
    #[serde(rename = "testIn")]
    test_in: String,
    // Ajoutez d'autres champs si nécessaires
}

fn load_testcase<P: AsRef<Path>>(testcase: P) -> Vec<CheckPoint> {
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

        for _ in 0..3 {
            for checkpoint in all_pts[1..].iter() {
                checkpoints.push(checkpoint.clone_checkpoint());
            }
            checkpoints.push(all_pts[0].clone_checkpoint());
        }

        let n_minus2 = &checkpoints[checkpoints.len() - 2];
        let n_minus1 = &checkpoints[checkpoints.len() - 1];

        // Conversion en Point pour utiliser les opérations
        let n_minus2_point = Point::from_f64(n_minus2.x, n_minus2.y);
        let n_minus1_point = Point::from_f64(n_minus1.x, n_minus1.y);

        let dist = n_minus2.distance(&n_minus1_point);
        let factor = 50_000.0 / dist;

        // Calcul du dernier point
        // last_pt = n_minus1 * (factor+1) - n_minus2 * factor
        let n_minus1_scaled = &n_minus1_point * (factor + 1.0);
        let n_minus2_scaled = &n_minus2_point * factor;
        let last_pt = Point::from_f64(
            (n_minus1_scaled.x - n_minus2_scaled.x).trunc(),
            (n_minus1_scaled.y - n_minus2_scaled.y).trunc(),
        );

        checkpoints.push(CheckPoint::from_f64(last_pt.x, last_pt.y));
    }

    checkpoints
}

pub fn get_initial_pod(checkpoints: &[CheckPoint]) -> Pod {
    let starting_position = &checkpoints[checkpoints.len() - 2];
    let mut pod = Pod::new(starting_position.x, starting_position.y, 0.0, 0.0, 0.0, 0);

    let checkpoint_point = Point::from_f64(checkpoints[0].x, checkpoints[0].y);
    let angle = pod.get_angle(&checkpoint_point);
    pod.angle = angle.round();

    pod.done = false;
    pod.turn = 0;

    pod
}

fn main() {
    let checkpoints = load_testcase("testcases/test1.json");
    let base_pod = get_initial_pod(&checkpoints);

    let all_possible_actions: Vec<Action> = (0..=200)
        .flat_map(|thrust| (-18..=18).map(move |angle| Action::new(thrust, angle)))
        .collect();

    let mut best_score = 0.0;
    let mut best_action = Action::new(0, 0);
    for action in all_possible_actions.iter() {
        let mut pod = base_pod.clone_pod();
        pod.apply_move(action, &checkpoints);
        let score = pod.score(&checkpoints);

        if score > best_score {
            best_score = score;
            best_action = action.clone();
        }
    }
    println!("Best action: {:?}", best_action);

    // println!("Pod: {:?}", pod);
    // println!("Checkpoints: {:?}", checkpoints);

    // Vous pouvez ajouter d'autres logiques de jeu ici
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{action::Action, checkpoint::CheckPoint},
        get_initial_pod, load_testcase,
    };

    #[test]
    fn test_start_position() {
        let checkpoints = load_testcase("testcases/test1.json");
        let pod = get_initial_pod(&checkpoints);

        assert_eq!(pod.x, 10353.0);
        assert_eq!(pod.y, 1986.0);
        assert_eq!(pod.angle.round(), 161.0);
        assert_eq!(pod.vx, 0.0);
        assert_eq!(pod.vy, 0.0);
        assert_eq!(pod.next_checkpoint_id, 0);
    }

    #[test]
    fn test_checkpoint_list() {
        let checkpoints = load_testcase("testcases/test1.json");

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

        assert_eq!(checkpoints, expected);
    }

    #[test]
    fn test_apply_move() {
        let checkpoints = vec![
            CheckPoint::from_i32(800, 0),
            CheckPoint::from_i32(2200, 0),
            CheckPoint::from_i32(3600, 0),
        ];
        let mut pod = get_initial_pod(&checkpoints);

        assert_eq!(pod.turn, 0);

        pod.apply_move(&Action::new(1, 0), &checkpoints);
        assert_eq!(pod.turn, 1);
        assert!(!pod.done);

        for _ in 0..598 {
            pod.apply_move(&Action::new(1, 0), &checkpoints);
        }
        assert_eq!(pod.turn, 599);
        assert!(!pod.done);

        pod.apply_move(&Action::new(1, 0), &checkpoints);
        assert_eq!(pod.turn, 600);
        assert!(pod.done);
    }

    #[test]
    fn test_apply_moves() {
        let checkpoints = vec![
            CheckPoint::from_i32(800, 0),
            CheckPoint::from_i32(2200, 0),
            CheckPoint::from_i32(3600, 0),
        ];
        let mut pod = get_initial_pod(&checkpoints);

        assert_eq!(pod.turn, 0);

        pod.apply_move(&Action::new(1, 0), &checkpoints);
        assert_eq!(pod.turn, 1);
        assert!(!pod.done);

        let actions: Vec<Action> = (0..598).map(|_| Action::new(1, 0)).collect();
        pod.apply_moves(&actions, &checkpoints);
        assert_eq!(pod.turn, 599);
        assert!(!pod.done);

        pod.apply_move(&Action::new(1, 0), &checkpoints);
        assert_eq!(pod.turn, 600);
        assert!(pod.done);
    }

    #[test]
    fn test_simulation_1() {
        let checkpoints = load_testcase("testcases/test13.json");
        let mut pod = get_initial_pod(&checkpoints);

        assert_eq!(pod.x, 13332.0);
        assert_eq!(pod.y, 4114.0);
        assert_eq!(pod.angle.round(), 154.0);
        assert_eq!(pod.vx, 0.0);
        assert_eq!(pod.vy, 0.0);
        assert_eq!(pod.next_checkpoint_id, 0);

        let command = "200,-14;200,-1;200,14;200,7;200,-12;176,0;200,18;116,18;100,18;0,18;0,18;15,18;146,18;200,18;200,18;195,18;200,9;200,6;200,15;200,15;200,11;200,11;161,-3;191,18;98,18;18,18;0,18;20,18;87,11;200,18;197,18;200,18;200,18;200,14;151,-7;166,9;200,-8;200,10;197,-1;200,7;200,-7;200,-18;200,-18;123,-18;7,-18;0,-18;0,-18;44,-18;58,-18;158,-18;176,-18;200,-18;200,-18;200,-17;200,-15;200,-2;200,0;200,9;198,-9;184,17;184,8;173,18;157,18;120,12;0,18;178,18;131,16;200,15;200,18;200,15;200,11;200,-9;200,9;200,4;194,10;200,14;171,16;200,15;16,18;53,18;0,18;200,18;200,15;200,18;194,18;200,12;200,14;176,8;143,15;188,18;192,-18;200,1;157,-18;11,-18;13,-18;0,-18;0,-18;3,-18;200,-18;200,-18;200,-16;200,-18;200,-3;200,-11;200,-18;200,6;200,10;200,2;200,-4;191,0;200,-18;200,-18;195,-18;37,-16;27,-18;0,-18;22,-18;5,-18;103,-18;200,-18;197,-9;200,-8;200,-6;200,-18;200,2;200,-5;200,6;194,11;200,18;200,8;200,18;149,9;91,18;64,14;114,13;160,18;200,18;200,18;200,11;190,-13;200,18;200,11;200,3;187,-4;200,18;198,18;92,18;90,18;12,18;174,18;199,18;200,18;200,14;200,18;200,14;200,3;200,18;199,1;200,15;200,5;193,15;92,18;0,18;10,18;0,18;43,18;200,18;200,18;200,18;200,11;200,0;198,0;200,18;200,-6;200,11;200,-11;200,-9;185,-10;198,-8;176,-12;37,-18;9,-18;0,-18;0,-18;48,-18;145,-18;200,-18;200,-18;200,-18;200,-18;200,-18;200,-18;200,18;149,-17;199,7;71,-10;179,5;169,18;157,18;139,10;185,16;187,16;176,15;0,18;200,18;200,18;200,0;200,-8";

        let actions: Vec<Action> = command.split(';').map(Action::from).collect();

        for (i, action) in actions.iter().enumerate() {
            let t = pod.apply_move(action, &checkpoints);
            if pod.done {
                let error_score = i as f64 + t - 207.57;
                assert!(error_score.abs() < 0.1, "Error score: {}", error_score);
                return;
            }
        }

        panic!("Game should have ended before the last action");
    }
}
