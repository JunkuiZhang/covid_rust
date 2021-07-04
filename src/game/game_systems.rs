use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;

use crate::game::game_data::Vector;
use crate::settings::{INFECT_RADIUS, POP_NUM, WINDOW_WIDTH};

use super::game_data::EntityStatus;

pub fn entity_decision(
    self_num: usize,
    pos: &mut Vector,
    dir: &mut Vector,
    duration: f64,
    status: &EntityStatus,
    neighbor_list: &[Vector],
    rng: &mut ThreadRng,
) {
    let mut dir_res;
    if status.is_aware {
        match entity_get_dir(self_num, pos, neighbor_list) {
            Some(some_dir) => {
                *dir = some_dir;
            }
            None => {
                if rng.gen_bool(0.1) {
                    dir_res = Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                    dir_res.normalize();
                    dir_res += *dir;
                    dir_res.normalize();
                    *dir = dir_res;
                }
            }
        }
    } else {
        if rng.gen_bool(0.1) {
            dir_res = Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            dir_res.normalize();
            dir_res += *dir;
            dir_res.normalize();
            *dir = dir_res
        }
    }
    entity_move(pos, dir, duration)
}

pub fn entity_move(pos: &mut Vector, dir: &Vector, duration: f64) {
    *pos += *dir * duration * 50.0;
    if pos.x < 0.0 {
        pos.x += WINDOW_WIDTH as f64;
    }
    if pos.x > WINDOW_WIDTH as f64 {
        pos.x -= WINDOW_WIDTH as f64;
    }
    if pos.y < 0.0 {
        pos.y += WINDOW_WIDTH as f64;
    }
    if pos.y > WINDOW_WIDTH as f64 {
        pos.y -= WINDOW_WIDTH as f64;
    }
}

pub fn entity_color(entity_status: &EntityStatus) -> Color {
    if entity_status.is_aware {
        return Color::RGB(0, 0, 170);
    }
    if entity_status.is_infected {
        return Color::RGB(170, 0, 0);
    }
    return Color::RGB(100, 100, 100);
}

pub fn entity_get_dir(self_num: usize, pos: &Vector, neighbor_list: &[Vector]) -> Option<Vector> {
    let mut neighbor_near = Vec::new();
    for i in 0..POP_NUM as usize {
        if i == self_num {
            continue;
        }
        if pos.distance_with(&neighbor_list[i]) < INFECT_RADIUS * 1.5 {
            neighbor_near.push(*pos - neighbor_list[i]);
        }
    }

    if neighbor_near.len() > 0 {
        let mut dir = Vector::new(0.0, 0.0);
        for vec in neighbor_near {
            dir += vec;
        }
        dir.normalize();
        return Some(dir);
    } else {
        return None;
    }
}
