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
    time : f32,
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
                time : 0.0,
            };
            world.init();
            
            world
        }
    
    fn init(&mut self) {
  
    }


    pub fn step(&mut self, dt:f32){

        let mut hold = false;
        self.time += dt; 



        // update physics
        for c in self.creatures.iter_mut() {

            if is_mouse_button_down(MouseButton::Left){
                c.joints[0].hold = true;
            }

            for p in c.particles.iter_mut(){

                p.step(dt, self.gravity);

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
        // update creatures
        for c in  self.creatures.iter_mut() {
            c.update(self.time);
            c.joints[0].hold = false;

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