use macroquad::prelude::*;

mod world;
use world::World;

mod creature;
use creature::Creature;

mod camera;

#[macroquad::main("Climbing Sim")]
async fn main() {
    // limb_length [forarm, arm, tigh, calf]
    // skel_length [clavicle, hip, trunc]

    let limb_length = vec![100.0,100.0, 100.0, 100.0];
    let skel_length = vec![20.0, 35.0, 200.0];

    let creatures = vec![Creature::new(RED, limb_length, skel_length)];

    let w = screen_width();
    let h = screen_height();

    let mut world = World::new(
        creatures,
        10.0,
        7.0,
        0.8,
        0.95,
        10,
        w,
        h,

    );

    loop {
        let dt = get_frame_time();


        world.step(dt);
        world.draw();

        next_frame().await;

    }
}