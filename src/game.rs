use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod game_systems;
mod game_data;


pub struct Game {
    // Info
    is_running: bool,

    // Systems
    events_pump: EventPump,
    canvas: WindowCanvas,
}

impl Game {
    pub fn new(width: u32, title: &str) -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let game_window = video_subsystem.window(title, width, width)
            .position_centered()
            .build()
            .unwrap();
        let events_pump = sdl_context.event_pump().unwrap();
        let canvas = game_window.into_canvas()
            .build()
            .unwrap();
        Game {
            is_running: true,
            events_pump,
            canvas,
        }
    }

    pub fn run(&mut self) {
        self.pre_render();
        while self.is_running {
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
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_running = false;
                },
                _ => {},
            }
        }
    }

    fn update_render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
}
