use std::time::SystemTime;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use self::game_data::{EntityStatus, Vector};
use self::game_status::GameStatus;
use crate::game::game_systems::{entity_circle_color, entity_color, entity_decision};
use crate::settings::{
    INFECT_RADIUS, INITIAL_AWARE_PARTION, INITIAL_INFECTED_PARTION, POP_NUM, WINDOW_WIDTH,
};

mod game_data;
mod game_status;
mod game_systems;

pub(crate) struct Game {
    // Info
    status: GameStatus,
    running: bool,
    time_stamp: SystemTime,
    rng: ThreadRng,
    fps: u32,

    // Systems
    events_pump: EventPump,
    canvas: WindowCanvas,

    // Data
    position_vector_list: [Vector; POP_NUM as usize],
    pos_vec_copy: [Vector; POP_NUM as usize],
    status_list: [EntityStatus; POP_NUM as usize],
    direction_vector_list: [Vector; POP_NUM as usize],
}

impl Game {
    pub fn new(width: u32, title: &str) -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let game_window = video_subsystem
            .window(title, width, width)
            .position_centered()
            .build()
            .unwrap();
        let events_pump = sdl_context.event_pump().unwrap();
        let canvas = game_window.into_canvas().build().unwrap();

        let time_stamp = SystemTime::now();

        let mut rng = rand::thread_rng();
        let mut position_vector_list = [Vector::new(0.0, 0.0); POP_NUM as usize];
        let mut status_list = [EntityStatus {
            is_alive: true,
            is_aware: false,
            is_infected: false,
        }; POP_NUM as usize];
        let mut direction_vector_list = [Vector::new(0.0, 0.0); POP_NUM as usize];
        for num in 0..POP_NUM as usize {
            position_vector_list[num] = Vector::new(
                rng.gen_range(0.0..1.0) * WINDOW_WIDTH as f64,
                rng.gen_range(0.0..1.0) * WINDOW_WIDTH as f64,
            );
            if rng.gen_bool(INITIAL_AWARE_PARTION) {
                status_list[num].is_aware = true;
            }
            if rng.gen_bool(INITIAL_INFECTED_PARTION) {
                status_list[num].is_infected = true;
            }
            let mut dir = Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            dir.normalize();
            direction_vector_list[num] = dir;
        }
        let pos_vec_copy = position_vector_list.clone();

        Game {
            status: GameStatus::Paused,
            running: true,
            time_stamp,
            rng,
            fps: 0,
            events_pump,
            canvas,
            position_vector_list,
            pos_vec_copy,
            status_list,
            direction_vector_list,
        }
    }

    pub fn run(&mut self) {
        self.pre_render();
        while self.running {
            self.events();
            self.update_render();
        }
    }

    fn pre_render(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();
    }

    fn events(&mut self) {
        for event in self.events_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self.status.toggle();
                }
                _ => {}
            }
        }
    }

    fn update_render(&mut self) {
        let start_time;
        let duration;
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        match self.status {
            GameStatus::Running => {
                start_time = SystemTime::now();
                duration = start_time
                    .duration_since(self.time_stamp)
                    .unwrap()
                    .as_secs_f64();
                for num in 0..POP_NUM as usize {
                    let mut entity_status = self.status_list[num].clone();
                    if !entity_status.is_alive {
                        continue;
                    }
                    let pos_vec = &mut self.position_vector_list[num];
                    let dir_vec = &mut self.direction_vector_list[num];
                    entity_decision(
                        num,
                        pos_vec,
                        dir_vec,
                        duration,
                        &mut entity_status,
                        &self.status_list,
                        &self.pos_vec_copy,
                        &mut self.rng,
                    );
                    self.status_list[num] = entity_status;
                    let (x, y) = pos_vec.get_nums();
                    self.canvas
                        .filled_circle(x, y, 5, entity_color(&entity_status))
                        .unwrap();
                    if let Some(color) = entity_circle_color(&entity_status) {
                        self.canvas
                            .circle(x, y, INFECT_RADIUS.round() as i16, color)
                            .unwrap();
                    }
                }
                self.pos_vec_copy = self.position_vector_list.clone();
            }
            GameStatus::Paused => {
                start_time = SystemTime::now();
                duration = start_time
                    .duration_since(self.time_stamp)
                    .unwrap()
                    .as_secs_f64();
                for num in 0..POP_NUM as usize {
                    let (x, y) = self.position_vector_list[num].get_nums();
                    let entity_status = &self.status_list[num];
                    self.canvas
                        .filled_circle(x, y, 5, entity_color(entity_status))
                        .unwrap();
                    if let Some(color) = entity_circle_color(entity_status) {
                        self.canvas
                            .circle(x, y, INFECT_RADIUS.round() as i16, color)
                            .unwrap();
                    }
                }
            }
        }
        self.canvas.present();
        let current_fps = (1.0 / duration).round();
        self.fps += current_fps as u32;
        self.fps /= 2;
        println!("AVE FPS: {}, FPS: {}", self.fps, current_fps);
        self.time_stamp = start_time;
    }
}
