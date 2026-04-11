use macroquad::prelude::*;


use climbing_sim::sim::Sim;
use climbing_sim::features::Features;
use climbing_sim::features::gen_random_features;
use climbing_sim::camera::Camera2DWorld;



#[macroquad::main("Climbing Sim")]
async fn main() {

    // init
    let cam = Camera2DWorld::new();

    
    let sim_count = 3;
    let time_sim = 5.0;

    for i in 0..sim_count {
    
        print!("Launch sim {}\n", i);
        let features_lst = (0..2).map(|_| gen_random_features()).collect();
        let mut sim = Sim::new(features_lst, cam);            
        
        let mut elapsed_time = 0.0;
        loop {
            let dt = get_frame_time();
            elapsed_time += dt;
            if elapsed_time >= time_sim {
                print!("Sim {} finished\n", i);
                break;
            }

            
            sim.step(dt);
            sim.draw();

            next_frame().await;
        }
    }

}