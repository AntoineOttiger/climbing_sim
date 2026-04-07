use macroquad::prelude::*;

use crate::creature::Creature;
use crate::creature::Particle;

fn collide(a: &mut Particle, b: &mut Particle, radius: f32, restitution: f32) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    let dist2 = dx * dx + dy * dy;
    let min_dist = 2.0 * radius;

    if dist2 >= min_dist * min_dist {
        return;
    }

    let dist = dist2.sqrt();

    // Évite une division par zéro si deux centres sont exactement superposés
    let (nx, ny, dist) = if dist > 1e-6 {
        (dx / dist, dy / dist, dist)
    } else {
        (1.0, 0.0, min_dist)
    };

    // --- 1) Correction de position ---
    let overlap = min_dist - dist;
    let correction = overlap * 0.5;

    a.x -= nx * correction;
    a.y -= ny * correction;
    b.x += nx * correction;
    b.y += ny * correction;

    // --- 2) Correction de vitesse ---
    let rvx = b.vx - a.vx;
    let rvy = b.vy - a.vy;

    let vel_along_normal = rvx * nx + rvy * ny;

    // Si elles s'éloignent déjà, ne rien faire
    if vel_along_normal > 0.0 {
        return;
    }

    // masses égales = formule simple
    let impulse = -(1.0 + restitution) * vel_along_normal / 2.0;

    let ix = impulse * nx;
    let iy = impulse * ny;

    a.vx -= ix;
    a.vy -= iy;
    b.vx += ix;
    b.vy += iy;
}


pub struct World {
    creatures : Vec<Creature>,
    gravity: f32,
    radius: f32,
    wall_restitution: f32,
    particle_restitution: f32,
    solver_iterations: i16,
    w : f32,
    h : f32,
}

impl World{

    pub fn new(
            creatures : Vec<Creature>,
            gravity: f32,
            radius: f32,
            wall_restitution: f32,
            particle_restitution: f32,
            solver_iterations: i16,
            w : f32,
            h : f32,
        ) -> Self {
            let mut world  = Self {
                creatures,
                gravity,
                radius,
                wall_restitution,
                particle_restitution,
                solver_iterations,
                w,
                h,
            };
            world.init();
            
            world
        }
    
    fn init(&mut self) {
  
    }


    pub fn step(&mut self, dt:f32){

        let mut hold = false;



        // update physics
        for c in self.creatures.iter_mut() {
            let mut _counter = 0;
            for p in c.particles.iter_mut(){
                if is_mouse_button_down(MouseButton::Left) && _counter==0{
                    hold = true;
                }

                p.step(dt, self.gravity, hold);
                hold = false;

                if p.x - self.radius <= 0.0 {
                    p.x = self.radius;
                    p.vx = -p.vx * self.wall_restitution;
                }

                if p.x + self.radius >= self.w {
                    p.x = self.w - self.radius;
                    p.vx = -p.vx * self.wall_restitution;
                }

                if p.y - self.radius <= 0.0 {
                    p.y = self.radius;
                    p.vy = -p.vy * self.wall_restitution;
                }

                if p.y + self.radius >= self.h {
                    p.y = self.h - self.radius;
                    p.vy = -p.vy * self.wall_restitution;
                }
                _counter += 1;
            
            }

        }

        // solve collisions
        for _ in 0..self.solver_iterations {
            for c in  self.creatures.iter_mut() {
                let len = c.particles.len();
                for i in 0..len {
                    for j in (i + 1)..len {
                        let (left, right) = c.particles.split_at_mut(j);
                        let a = &mut left[i];
                        let b = &mut right[0];

                        collide(a, b, self.radius, self.particle_restitution);
                    }
                }
            }
        }
        // update links
        for c in  self.creatures.iter_mut() {
            for link in &mut c.links{
                link.update(&mut c.particles);

            }
        }

        // update joints
        for c in  self.creatures.iter_mut() {
            for (join, pos) in c.joints.iter_mut().zip(c.target_pos.iter()){
                join.update(&mut c.particles, pos[0], pos[1], 0.2);
            }
        }


    }

    pub fn draw(&mut self){
        clear_background(BLACK);
        for c in  self.creatures.iter_mut() {
            for link in &mut c.links{
                link.draw(&mut c.particles);
            }

            for p in &c.particles {
                draw_circle(p.x, p.y, self.radius, p.color);
            }        
        }
    }
}