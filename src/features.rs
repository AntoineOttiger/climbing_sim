

use rand::Rng;

use crate::features;

#[derive(Clone)]
pub struct Features {
    pub target_pos_lst: Vec<Vec<Vec<f32>>>,
    pub hold_lst :Vec<Vec<bool>>,
    pub pos_time : f32,
}

/*
fn get_new_features(features : Features)->Features{

    for target_pos in features.target_pos_lst {
        for elem in target_pos{
            for coordinate in elem{

            }
        }
    }

}
*/

fn random_point_quarter_circle(radius: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let u: f32 = rng.r#gen();
    let theta: f32 = rng.gen_range(0.0..std::f32::consts::FRAC_PI_2);

    let r = radius * u.sqrt();

    let x = r * theta.cos();
    let y = r * theta.sin();

    (x, y)
}

fn get_rand_float(min : f32, max : f32)->f32{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

fn get_rand_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.r#gen()
}

pub fn gen_random_features()->Features{

    let mut target_pos_lst = vec![];
    let mut hold_lst = vec![];
    let pos_time = get_rand_float(0.3, 2.0);
    
    for _ in 0..4{
        let mut target_pos = vec![];
        let rand = random_point_quarter_circle(200.0);
        let top_left = vec![-rand.0, -rand.1]; 
        target_pos.push(top_left);
        
        let rand = random_point_quarter_circle(200.0);
        let top_right = vec![rand.0, -rand.1]; 
        target_pos.push(top_right);

        let rand = random_point_quarter_circle(200.0);
        let bot_left = vec![-rand.0, rand.1];
        target_pos.push(bot_left);
        
        let rand = random_point_quarter_circle(200.0);
        let top_right= vec![rand.0, rand.1];
        target_pos.push(top_right);
        target_pos_lst.push(target_pos);

        let hold: Vec<bool> = (0..4).map(|_| get_rand_bool()).collect();
        hold_lst.push(hold);

    }



    return Features{ target_pos_lst: target_pos_lst, hold_lst :hold_lst, pos_time :pos_time};
}