use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;

use crate::game::game_data::Vector;
use crate::settings::AWARE_RADIUS;
use crate::settings::INFECTION_CHANCE;
use crate::settings::NOT_AWARE_ENTITY_MOVE_MUL;
use crate::settings::{INFECT_RADIUS, POP_NUM, WINDOW_WIDTH};

use super::game_data::Awareness;
use super::game_data::EntityStatus;

pub fn entity_decision(
    self_num: usize,
    pos: &mut Vector,
    dir: &mut Vector,
    duration: f64,
    status: &mut EntityStatus,
    status_list: &[EntityStatus],
    neighbor_list: &[Vector],
    rng: &mut ThreadRng,
) {
    let (near_list, infection_meet, aware_meet) =
        entity_get_nears(self_num, pos, neighbor_list, status_list);
    let mut speed;
    match status.is_aware {
        Awareness::Aware(s) => {
            speed = s;
            if aware_meet {
                speed *= NOT_AWARE_ENTITY_MOVE_MUL;
            }
            match entity_get_dir(&near_list) {
                Some(some_dir) => {
                    *dir = some_dir;
                }
                None => {
                    entity_generate_new_dir(dir, rng);
                }
            }
        }
        Awareness::NotAware(s) => {
            speed = s;
            entity_generate_new_dir(dir, rng);
        }
    }
    entity_move(pos, dir, duration, speed);
    if infection_meet && !status.is_infected {
        if rng.gen_bool(INFECTION_CHANCE) {
            status.is_infected = true;
        }
    }
}

fn entity_generate_new_dir(dir: &mut Vector, rng: &mut ThreadRng) {
    if rng.gen_bool(0.1) {
        let mut dir_res = Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        dir_res.normalize();
        dir_res += *dir;
        dir_res.normalize();
        *dir = dir_res;
    }
}

pub fn entity_move(pos: &mut Vector, dir: &Vector, duration: f64, speed: f64) {
    *pos += *dir * duration * speed;
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
    let mut color = Color::RGB(100, 100, 100);
    if let Awareness::Aware(_) = entity_status.is_aware {
        color = Color::RGB(0, 0, 170);
    }
    if entity_status.is_infected {
        color = Color::RGB(170, 0, 0);
    }
    return color;
}

pub fn entity_circle_color(entity_status: &EntityStatus) -> Option<Color> {
    let mut color = None;
    if entity_status.is_infected {
        color = Some(Color::RGB(170, 0, 0));
        if let Awareness::Aware(_) = entity_status.is_aware {
            color = Some(Color::RGB(0, 0, 170));
        }
    }
    return color;
}

fn entity_get_nears(
    self_num: usize,
    pos: &Vector,
    neighbor_list: &[Vector],
    status_list: &[EntityStatus],
) -> (Vec<Vector>, bool, bool) {
    let mut near_list_aware_radius = Vec::new();
    let mut infection_meet = false;
    let mut aware_meet = false;
    for i in 0..POP_NUM as usize {
        if i == self_num {
            continue;
        }
        let dist = pos.distance_with(&neighbor_list[i]);
        if dist > AWARE_RADIUS {
            continue;
        }
        near_list_aware_radius.push((*pos - neighbor_list[i]) * get_weight(dist));
        aware_meet = true;
        if dist < INFECT_RADIUS && status_list[i].is_infected {
            infection_meet = true;
        }
    }
    return (near_list_aware_radius, infection_meet, aware_meet);
}

pub fn entity_get_dir(near_list: &Vec<Vector>) -> Option<Vector> {
    if near_list.len() == 0 {
        return None;
    }
    let mut dir = Vector::new(0.0, 0.0);
    for vec in near_list {
        dir += *vec;
    }
    dir.normalize();
    return Some(dir);
}

fn get_weight(dist: f64) -> f64 {
    return (AWARE_RADIUS - dist).powi(2);
}
