use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
}

impl Particle {


    pub fn step(&mut self, dt: f32, g: f32, hold:bool) {
        if hold {
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
    c: usize
}

impl Joint {
    pub fn update(
        &mut self,
        particles: &mut Vec<Particle>,
        target_rel_ax: f32,
        target_rel_ay: f32,
        strength: f32,
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
        particles[self.a].vx += fx;
        particles[self.a].vy += fy;

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
    
    pub fn draw(&mut self, particles : &mut Vec<Particle>){
        
        if self.visible{
            let thickness = 5.0;
            let color = GREEN;
            
            let a_x = particles[self.a].x;
            let a_y = particles[self.a].y;
            let b_x = particles[self.b].x;
            let b_y = particles[self.b].y;

            
            draw_line(a_x, a_y, b_x, b_y, thickness, color);

        }

    }
}

pub struct  Creature {
    color : Color,
    limb_length : Vec<f32>,
    skel_length :Vec<f32>,
    
    pub particles: Vec<Particle>,
    pub links:Vec<Link>,
    pub joints: Vec<Joint>,

    // [top_left, top_right, bottom_left, bottom_right]
    pub target_pos_lst: Vec<Vec<Vec<f32>>>,
    pos_time : f32,
}

impl Creature {
    pub fn new(
        color : Color,
        limb_length :Vec<f32>,
        skel_length :Vec<f32>,


    ) -> Self {
        let mut creature = Self {
            color,
            limb_length : limb_length,
            skel_length : skel_length,


            particles : vec![],
            links : vec![],
            joints : vec![],
            target_pos_lst : vec![],
            pos_time : 5.0,

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

        let pos_x = 100.0;
        let pos_y = 100.0; 

        // left arm
        self.particles.push(Particle {x: pos_x, y: pos_y, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x, y: pos_y+forarm, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+arm, y: pos_y+forarm, vx: 0.0, vy: 0.0, color: self.color});

        // right arm
        self.particles.push(Particle {x: pos_x+2.0*arm+2.0*clavicle, y: pos_y, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+2.0*arm+2.0*clavicle, y: pos_y+forarm, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+arm+2.0*clavicle, y: pos_y+forarm, vx: 0.0, vy: 0.0, color: self.color});

        // left leg
        self.particles.push(Particle {x: pos_x+clavicle-hip, y: pos_y+forarm+trunc+calf, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+clavicle-hip, y: pos_y+forarm+trunc, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+clavicle-hip+tigh, y: pos_y+forarm+trunc, vx: 0.0, vy: 0.0, color: self.color});

        // right leg
        self.particles.push(Particle {x: pos_x+2.0*tigh+2.0*hip+clavicle-hip, y: pos_y+forarm+trunc+calf, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+2.0*tigh+2.0*hip+clavicle-hip, y: pos_y+forarm+trunc, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+tigh+2.0*hip+clavicle-hip, y: pos_y+forarm+trunc, vx: 0.0, vy: 0.0, color: self.color});

        // skeleton
        self.particles.push(Particle {x: pos_x+arm+clavicle, y: pos_y+forarm+trunc, vx: 0.0, vy: 0.0, color: self.color});
        self.particles.push(Particle {x: pos_x+arm+clavicle, y: pos_y+forarm, vx: 0.0, vy: 0.0, color: self.color});
        

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
        self.joints.push(Joint {a : 0, b : 1, c: 2});
        self.joints.push(Joint {a : 3, b : 4, c: 5});
        self.joints.push(Joint {a : 6, b : 7, c: 8});
        self.joints.push(Joint {a : 9, b : 10, c: 11});
        
        // set target positions
        //self.target_pos = vec![vec![-50.0, -50.0], vec![50.0, -50.0], vec![-50.0, 50.0], vec![50.0, 50.0]];
        let target_pos_1 = vec![vec![-50.0, -50.0], vec![50.0, -50.0], vec![-50.0, 50.0], vec![50.0, 50.0]];
        let target_pos_2 = vec![vec![-25.0, -50.0], vec![50.0, -50.0], vec![-50.0, 50.0], vec![50.0, 50.0]];
        self.target_pos_lst = vec![target_pos_1, target_pos_2];
    }

    pub fn update(&mut self, time:f32) {
        

        let trg_pos_idx = ((time / self.pos_time) as usize) % self.target_pos_lst.len();  
        let target_pos = &self.target_pos_lst[trg_pos_idx];
        // update links
        for link in &mut self.links {
            link.update(&mut self.particles);
        }

        // update joints
        for (join, pos) in self.joints.iter_mut().zip(target_pos.iter()){   
            join.update(&mut self.particles, pos[0], pos[1], 0.2);
        }
    }

}