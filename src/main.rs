mod game;

use game::game_manager::GameManager;

fn main() {
    let mut game_manager = GameManager::new(100);
    let (pod, checkpoints) = game_manager.set_testcase("testcases/test1.json");

    println!("Pod: {:?}", pod);
    println!("Checkpoints: {:?}", checkpoints);

    // Vous pouvez ajouter d'autres logiques de jeu ici
}
