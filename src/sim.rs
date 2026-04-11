use macroquad::prelude::*;

use crate::world::World;
use crate::camera::Camera2DWorld;
use crate::creature::Creature;
use crate::features::Features;

pub struct Sim{
    features_lst : Vec<Features>,
    cam : Camera2DWorld,

    time_sim : f32,
    world_lst : Vec<World>,
}


impl Sim {

    pub fn new(
            features_lst : Vec<Features>,
            cam : Camera2DWorld,

        ) -> Self {
            let mut sim  = Self {
                features_lst : features_lst,
                cam : cam,
                time_sim : 10.0,
                world_lst : vec![],
            };
            sim.init();
            
            sim
        }

    fn init(&mut self){

        for n in 0..self.features_lst.len(){
            let i = n as f32;

            let init_pos = Vec2{x : 100.0 + i*200.0, y : 100.0};
            let features = &self.features_lst[n];

            let world = self.create_world_instance(init_pos, features.clone());
            self.world_lst.push(world);

        }

    }
    
    pub fn get_height_scores(&mut self) -> Vec<f32> {
        self.world_lst
            .iter_mut()  // ← iter_mut au lieu de iter
            .map(|world| world.retrieve_creatures_heights()[0])// one creature per world
            .collect()
    }

    pub fn step(&mut self, dt : f32){
        for world in &mut self.world_lst{
            world.step(dt);
        }
    }

    pub fn draw(&mut self){
        clear_background(BLACK);
        for world in &mut self.world_lst{
            world.draw();
        }
    } 

    fn create_world_instance(&mut self, init_pos : Vec2, features : Features) -> World {

        // limb_length [forarm, arm, tigh, calf]
        // skel_length [clavicle, hip, trunc]
        let limb_length = vec![100.0,100.0, 100.0, 100.0];
        let skel_length = vec![20.0, 35.0, 200.0];

        let creatures = vec![Creature::new(RED, limb_length, skel_length, init_pos, features)];

        let w = screen_width();
        let h = screen_height();
        let cam = Camera2DWorld::new();

        World::new(
            creatures,
            500.0,
            7.0,
            0.8,
            0.95,
            10,
            w,
            h,
            cam,

        )

    }
}