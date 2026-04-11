use macroquad::prelude::*;


use climbing_sim::sim::Sim;
use climbing_sim::features::Features;
use climbing_sim::camera::Camera2DWorld;




#[macroquad::main("Climbing Sim")]
async fn main() {

    let target_pos_1 = vec![vec![-50.0, -50.0], vec![50.0, -50.0], vec![-50.0, 50.0], vec![50.0, 50.0]];
    let target_pos_2 = vec![vec![-25.0, -70.0], vec![50.0, -50.0], vec![-50.0, 50.0], vec![-50.0, 50.0]];
    let target_pos_lst = vec![target_pos_1, target_pos_2];
    let hold_1 = vec![true, false, false, false];
    let hold_2 = vec![false, true, false, false];
    let hold_lst = vec![hold_1, hold_2];
    let features = Features{target_pos_lst : target_pos_lst, hold_lst : hold_lst, pos_time : 0.5};
    let mut features_lst = vec![];
    features_lst.push(features.clone());
    features_lst.push(features.clone());
    
    let cam = Camera2DWorld::new();
    let mut sim = Sim::new(features_lst, cam);
    
    loop {
        let dt = get_frame_time();
        sim.step(dt);
        sim.draw();

        next_frame().await;

    }

}