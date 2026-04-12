use macroquad::prelude::*;

use ::rand::Rng;

use crate::features;

#[derive(Clone)]
pub struct Features {
    pub target_pos_lst: Vec<Vec<Vec2>>,
    pub hold_lst :Vec<Vec<bool>>,
    pub pos_time : f32,
}


fn get_new_features(features: Features) -> Features {
    let mut target_pos_lst = vec![];
    let mut hold_lst = vec![];
    let pos_time  = features.pos_time * ::rand::thread_rng().gen_range(0.9..1.1);
    
    for target_pos in features.target_pos_lst.iter() {
        let mut new_target_pos = vec![];
        
        for (index, pos) in target_pos.iter().enumerate() {

            if index == 0 {
                let new_pos = perturb_point_quarter_circle(-pos.x, -pos.y, 200.0, 10.0);
                let new_pos_vec = vec2(-new_pos.0, -new_pos.1);
                new_target_pos.push(new_pos_vec);
            }
            else if index == 1 {
                let new_pos = perturb_point_quarter_circle(pos.x, -pos.y, 200.0, 10.0);
                let new_pos_vec = vec2(new_pos.0, -new_pos.1);
                new_target_pos.push(new_pos_vec);                
            }
            else if index == 2 {
                let new_pos = perturb_point_quarter_circle(-pos.x, pos.y, 200.0, 10.0);
                let new_pos_vec = vec2(-new_pos.0, new_pos.1);
                new_target_pos.push(new_pos_vec);                
            }
            else if index == 3 {
                let new_pos = perturb_point_quarter_circle(pos.x, pos.y, 200.0, 10.0);
                let new_pos_vec = vec2(new_pos.0, new_pos.1);
                new_target_pos.push(new_pos_vec);               
                
            }
            else {
                print!("Error");
            }

        
        }
        
        target_pos_lst.push(new_target_pos);
    }
    
    
    Features {
        target_pos_lst,
        hold_lst,
        pos_time, 
    }
}




fn perturb_point_quarter_circle(x: f32, y: f32, radius: f32, max_perturbation: f32) -> (f32, f32) {
    let mut rng = ::rand::thread_rng();
    
    loop {
        // Génère une perturbation aléatoire dans les deux directions
        let dx: f32 = rng.gen_range(-max_perturbation..max_perturbation);
        let dy: f32 = rng.gen_range(-max_perturbation..max_perturbation);
        
        let new_x = x + dx;
        let new_y = y + dy;
        
        // Vérifie que le point reste dans le quart de cercle
        // Conditions: x >= 0, y >= 0, x² + y² <= radius²
        if new_x >= 0.0 && new_y >= 0.0 && (new_x * new_x + new_y * new_y) <= radius * radius {
            return (new_x, new_y);
        }
        
        // Si le point sort du quart de cercle, on réessaie
    }
}    

fn random_point_quarter_circle(radius: f32) -> (f32, f32) {
    let mut rng = ::rand::thread_rng();

    let u: f32 = rng.r#gen();
    let theta: f32 = rng.gen_range(0.0..std::f32::consts::FRAC_PI_2);

    let r = radius * u.sqrt();

    let x = r * theta.cos();
    let y = r * theta.sin();

    (x, y)
}

fn get_rand_float(min : f32, max : f32)->f32{
    let mut rng = ::rand::thread_rng();
    rng.gen_range(min..max)
}

fn get_rand_bool() -> bool {
    let mut rng = ::rand::thread_rng();
    rng.r#gen()
}

pub fn gen_random_features()->Features{

    let mut target_pos_lst = vec![];
    let mut hold_lst = vec![];
    let pos_time = get_rand_float(0.3, 2.0);
    
    for _ in 0..4{
        let mut target_pos = vec![];
        let rand = random_point_quarter_circle(200.0);
        let top_left = vec2(-rand.0,-rand.1); 
        target_pos.push(top_left);
        
        let rand = random_point_quarter_circle(200.0);
        let top_right = vec2(rand.0, -rand.1); 
        target_pos.push(top_right);

        let rand = random_point_quarter_circle(200.0);
        let bot_left = vec2(-rand.0, rand.1);
        target_pos.push(bot_left);
        
        let rand = random_point_quarter_circle(200.0);
        let top_right= vec2(rand.0, rand.1);
        target_pos.push(top_right);
        target_pos_lst.push(target_pos);

        let hold: Vec<bool> = (0..4).map(|_| get_rand_bool()).collect();
        hold_lst.push(hold);

    }



    return Features{ target_pos_lst: target_pos_lst, hold_lst :hold_lst, pos_time :pos_time};
}