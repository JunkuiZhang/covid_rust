use crate::game::game_data::Vector;
use crate::settings;
use crate::settings::WINDOW_WIDTH;

pub fn entity_move1(vec_list: &mut [Vector]) {
    for num in 0..settings::POP_NUM as usize {
        vec_list[num * 2] += vec_list[num * 2 + 1] * 3.0;
    }
}

pub fn entity_move2(pos_list: &mut [Vector], des_list: &[Vector], duration: f64) {
    for num in 0..settings::POP_NUM as usize {
        let mut pos_vec = pos_list[num];
        pos_vec += des_list[num] * duration;
        if pos_vec.x < 0.0 {
            pos_vec.x += WINDOW_WIDTH as f64;
        }
        if pos_vec.x > WINDOW_WIDTH as f64 {
            pos_vec.x -= WINDOW_WIDTH as f64;
        }
        if pos_vec.y < 0.0 {
            pos_vec.y += WINDOW_WIDTH as f64;
        }
        if pos_vec.y > WINDOW_WIDTH as f64 {
            pos_vec.y -= WINDOW_WIDTH as f64;
        }
        pos_list[num] = pos_vec;
    }
}
