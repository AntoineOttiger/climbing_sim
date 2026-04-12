use std::f32;


use macroquad::prelude::*;

use crate::camera::Camera2DWorld;
use crate::camera::draw_world_line;
use crate::features::Features;


#[derive(Copy, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub hold : bool,
}

impl Particle {

    pub fn new(
            x: f32,
            y: f32,
            vx: f32,
            vy: f32,
            color: Color,
        ) -> Self {
                let mut particle  = Self {
                    x,
                    y,
                    vx,
                    vy,
                    color,
                    hold:false,
                };
                particle.init();
                
                particle
            }
    
    fn init(&mut self) {
    }

    pub fn step(&mut self, dt: f32, g: f32) {
        if self.hold {
            self.vx = 0.0;
            self.vy = 0.0;
        }
        else {
            // apply gravity
            self.vy += g * dt;

            // update position
            self.x += self.vx * dt;
            self.y += self.vy * dt;
        }

    }
}

pub struct Joint {
    a: usize,
    b: usize,
    c: usize,
    pub hold: bool,
}

impl Joint {

    pub fn new(
        a: usize,
        b: usize,
        c: usize,          
        ) -> Self {
                let mut joint  = Self {
                    a,
                    b,
                    c,
                    hold:false,
                };
                joint.init();
                joint
            }
    
    fn init(&mut self) {
    }

    pub fn update(
        &mut self,
        particles: &mut Vec<Particle>,
        target_rel_ax: f32,
        target_rel_ay: f32,
        strength: f32,
        hold: bool
    ) {
        let ax = particles[self.a].x;
        let ay = particles[self.a].y;

        let bx = particles[self.b].x;
        let by = particles[self.b].y;

        let cx = particles[self.c].x;
        let cy = particles[self.c].y;

        // position cible absolue de A, définie relativement à C
        let target_ax = cx + target_rel_ax;
        let target_ay = cy + target_rel_ay;

        // erreur de position de A par rapport à sa cible
        let error_x = target_ax - ax;
        let error_y = target_ay - ay;

        // vitesse relative de A par rapport à C
        let rel_vx = particles[self.a].vx - particles[self.c].vx;
        let rel_vy = particles[self.a].vy - particles[self.c].vy;

        let damping = 0.1;

        // force de rappel + amortissement
        let fx = error_x * strength - rel_vx * damping;
        let fy = error_y * strength - rel_vy * damping;
        
        // on pousse A vers sa cible
        if hold {
            particles[self.a].vx = 0.0;
            particles[self.a].vy = 0.0;
            
        }
        else{
            particles[self.a].vx += fx;
            particles[self.a].vy += fy;
        }

        // réaction : on répartit sur B et C
        particles[self.b].vx -= fx * 0.5;
        particles[self.b].vy -= fy * 0.5;

        particles[self.c].vx -= fx * 0.5;
        particles[self.c].vy -= fy * 0.5;
    }
}

pub struct Link {
    a: usize,
    b: usize,
    rest_length: f32,
    visible : bool,
}

impl Link {

    pub fn update(&mut self, particles : &mut Vec<Particle>){
        
        let a_x = particles[self.a].x;
        let a_y = particles[self.a].y;
        let b_x = particles[self.b].x;
        let b_y = particles[self.b].y;

        let dx = b_x - a_x;
        let dy = b_y - a_y;

        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            return;
        }

        let dir_x = dx / dist;
        let dir_y = dy / dist;

        // Vitesse relative
        let rvx = particles[self.b].vx - particles[self.a].vx;
        let rvy = particles[self.b].vy - particles[self.a].vy;

        // Projection de la vitesse relative sur l'axe du lien
        let rel_vel_along_link = rvx * dir_x + rvy * dir_y;

        let k = 0.1;      // raideur du ressort
        let damping = 0.2; // amortissement

        let stretch = dist - self.rest_length;

        // Force ressort + amortissement
        let force = k * stretch + damping * rel_vel_along_link;

        let force_x = dir_x * force;
        let force_y = dir_y * force;

        particles[self.a].vx += force_x;
        particles[self.a].vy += force_y;

        particles[self.b].vx -= force_x;
        particles[self.b].vy -= force_y;
    }
    
    pub fn draw(&mut self, particles : &mut Vec<Particle>, cam : Camera2DWorld){
        
        if self.visible{
            
            let thickness = 5.0;
            let color = GREEN;
            

     
            draw_world_line(&cam, 
                            vec2(particles[self.a].x, particles[self.a].y), 
                            vec2(particles[self.b].x, particles[self.b].y), 
                            thickness, 
                            color);

        }

    }
}

pub struct  Creature {
    color : Color,
    limb_length : Vec<f32>,
    skel_length :Vec<f32>,
    init_pos : Vec2,
    features : Features,
    
    pub particles: Vec<Particle>,
    pub links:Vec<Link>,
    pub joints: Vec<Joint>,

    // [top_left, top_right, bottom_left, bottom_right]
    pub target_pos_lst: Vec<Vec<Vec2>>,
    pub hold_lst: Vec<Vec<bool>>,

}

impl Creature {
    pub fn new(
        color : Color,
        limb_length :Vec<f32>,
        skel_length :Vec<f32>,
        init_pos : Vec2,
        features : Features,

    ) -> Self {
        let mut creature = Self {
            color,
            limb_length : limb_length,
            skel_length : skel_length,
            init_pos : init_pos,
            features : features,


            particles : vec![],
            links : vec![],
            joints : vec![],
            target_pos_lst : vec![],
            hold_lst : vec![],


        };
        creature.init();

        creature
    }

    fn init(&mut self){
        
        // limb_length [forarm, arm, tigh, calf]
        // skel_length [clavicle, hip, trunc]

        let forarm = self.limb_length[0];
        let arm = self.limb_length[1];
        let tigh = self.limb_length[2];
        let calf = self.limb_length[3];

        let clavicle = self.skel_length[0];
        let hip = self.skel_length[1];
        let trunc = self.skel_length[2];
        let clavicule_support = (clavicle*clavicle + trunc*trunc).sqrt(); 
        let hip_support = (hip*hip + trunc*trunc).sqrt(); 

        let pos_x = self.init_pos.x;
        let pos_y = self.init_pos.y; 

        //left arm
        self.particles.push(Particle::new(pos_x, pos_y, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x, pos_y + forarm, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + arm, pos_y + forarm, 0.0, 0.0, self.color));

        // right arm
        self.particles.push(Particle::new(pos_x + 2.0 * arm + 2.0 * clavicle, pos_y, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + 2.0 * arm + 2.0 * clavicle, pos_y + forarm, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + arm + 2.0 * clavicle, pos_y + forarm, 0.0, 0.0, self.color));

        // left leg
        self.particles.push(Particle::new(pos_x + clavicle - hip, pos_y + forarm + trunc + calf, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + clavicle - hip, pos_y + forarm + trunc, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + clavicle - hip + tigh, pos_y + forarm + trunc, 0.0, 0.0, self.color));

        // right leg
        self.particles.push(Particle::new(pos_x + 2.0 * tigh + 2.0 * hip + clavicle - hip, pos_y + forarm + trunc + calf, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + 2.0 * tigh + 2.0 * hip + clavicle - hip, pos_y + forarm + trunc, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + tigh + 2.0 * hip + clavicle - hip, pos_y + forarm + trunc, 0.0, 0.0, self.color));

        // skeleton
        self.particles.push(Particle::new(pos_x + arm + clavicle, pos_y + forarm + trunc, 0.0, 0.0, self.color));
        self.particles.push(Particle::new(pos_x + arm + clavicle, pos_y + forarm, 0.0, 0.0, self.color));
        

        //set links
        //arms
        self.links.push(Link {a : 0, b : 1, rest_length : forarm, visible : true});
        self.links.push(Link {a : 1, b : 2, rest_length : arm, visible : true});
        self.links.push(Link {a : 3, b : 4, rest_length : forarm, visible : true});
        self.links.push(Link {a : 4, b : 5,rest_length : arm, visible : true});
        //legs
        self.links.push(Link {a : 6, b : 7, rest_length : calf, visible : true});
        self.links.push(Link {a : 7, b : 8, rest_length : tigh, visible : true});
        self.links.push(Link {a : 9, b : 10, rest_length : calf, visible : true});
        self.links.push(Link {a : 10, b : 11,rest_length : tigh, visible : true});
        //skeleton visible
        self.links.push(Link {a : 8, b : 12, rest_length : hip, visible : true});
        self.links.push(Link {a : 12, b : 11, rest_length : hip, visible : true});
        self.links.push(Link {a : 12, b : 13, rest_length : trunc, visible : true});
        self.links.push(Link {a : 2, b : 13, rest_length : clavicle, visible : true});
        self.links.push(Link {a : 13, b : 5, rest_length : clavicle, visible : true});
        // skeleton supports
        self.links.push(Link {a : 2, b : 12, rest_length : clavicule_support, visible : false});
        self.links.push(Link {a : 5, b : 12, rest_length : clavicule_support, visible : false});
        self.links.push(Link {a : 13, b : 8, rest_length : hip_support, visible : false});
        self.links.push(Link {a :13, b : 11, rest_length : hip_support, visible : false});
        self.links.push(Link {a :2, b : 8, rest_length : trunc, visible : false});
        self.links.push(Link {a :5, b : 11, rest_length : trunc, visible : false});
        //set joints
        self.joints.push(Joint::new(0, 1, 2));
        self.joints.push(Joint::new(3, 4, 5));
        self.joints.push(Joint::new(6, 7, 8));
        self.joints.push(Joint::new(9, 10, 11));
        
        // set target positions
        self.target_pos_lst = self.features.target_pos_lst.clone();
        
        // set hold list
        self.hold_lst = self.features.hold_lst.clone();    

    }

    pub fn update(&mut self, time:f32) {
        

        let trg_pos_idx = ((time / self.features.pos_time) as usize) % self.target_pos_lst.len();  
        let target_pos = &self.target_pos_lst[trg_pos_idx];
        let cur_hold = &self.hold_lst[trg_pos_idx]; 
        
        // update links
        for link in &mut self.links {
            link.update(&mut self.particles);
        }

        // update joints
        for ((join, pos), hold) in self.joints.iter_mut().zip(target_pos.iter()).zip(cur_hold.iter()){   
            join.update(&mut self.particles, pos[0], pos[1], 0.2, *hold);
        }
    }

}