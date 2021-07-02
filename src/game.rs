use std::time::SystemTime;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use self::game_data::Vector;
use self::game_status::GameStatus;
use self::game_systems::entity_move2;
use crate::settings::POP_NUM;
use crate::settings::WINDOW_WIDTH;

mod game_data;
mod game_status;
mod game_systems;

pub(crate) struct Game {
    // Info
    status: GameStatus,
    running: bool,
    time_stamp: SystemTime,

    // Systems
    events_pump: EventPump,
    canvas: WindowCanvas,

    // Data
    position_vector_list: [Vector; POP_NUM as usize],
    destination_vector_list: [Vector; POP_NUM as usize],
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
        let mut destination_vector_list = [Vector::new(0.0, 0.0); POP_NUM as usize];
        for num in 0..POP_NUM as usize {
            position_vector_list[num] = Vector::new(
                rng.gen_range(0.0..1.0) * WINDOW_WIDTH as f64,
                rng.gen_range(0.0..1.0) * WINDOW_WIDTH as f64,
            );
            let mut des = Vector::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
            des.normalize();
            destination_vector_list[num] = des;
        }

        Game {
            status: GameStatus::Paused,
            running: true,
            time_stamp,
            events_pump,
            canvas,
            position_vector_list,
            destination_vector_list,
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
        for num in 0..POP_NUM as usize {
            let (x, y) = self.position_vector_list[num].get_nums();
            self.canvas
                .filled_circle(x, y, 7, Color::RGB(0, 150, 0))
                .unwrap();
        }
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
                    self.status = self.status.toggle();
                }
                _ => {}
            }
        }
    }

    fn update_render(&mut self) {
        let start_time;
        let mut time_cost = 0.01;
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        match self.status {
            GameStatus::Running => {
                start_time = SystemTime::now();
                time_cost = start_time
                    .duration_since(self.time_stamp)
                    .unwrap()
                    .as_secs_f64();
                for num in 0..POP_NUM as usize {
                    entity_move2(
                        &mut self.position_vector_list,
                        &self.destination_vector_list,
                        time_cost,
                    );
                    let (x, y) = self.position_vector_list[num].get_nums();
                    self.canvas
                        .filled_circle(x, y, 5, Color::RGB(0, 150, 0))
                        .unwrap();
                }
            }
            GameStatus::Paused => {
                start_time = SystemTime::now();
                time_cost = start_time
                    .duration_since(self.time_stamp)
                    .unwrap()
                    .as_secs_f64();
                for num in 0..POP_NUM as usize {
                    let (x, y) = self.position_vector_list[num].get_nums();
                    self.canvas
                        .filled_circle(x, y, 5, Color::RGB(0, 150, 0))
                        .unwrap();
                }
            }
        }
        self.canvas.present();
        println!("FPS: {}", (1.0 / time_cost).round());
        self.time_stamp = start_time;
    }
}
