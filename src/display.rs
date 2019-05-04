use std::thread;
use std::time::Duration;

use crate::sdl2::pixels::{Color, PixelFormatEnum};
use crate::sdl2::event::{Event, WindowEvent};
use crate::sdl2::keyboard::Keycode;
use crate::sdl2::rect::Rect;

use crate::image::Image;


/// Holds the information in a display
/// 
/// This struct allows us to move event handling logic to methods,
/// and then check the state of the display after these methods,
/// in order to control flow.
struct Display {
    image: Image,
    width: u32,
    height: u32,
    should_end: bool
}

impl Display {
    fn new(image: Image) -> Display {
        let width = image.width as u32;
        let height = image.height as u32;
        Display { image, width, height, should_end: false }
    }

    fn handle(&mut self, event: Event) {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.should_end = true;
            },
            Event::Window{win_event, ..} => {
                self.handle_window(win_event);
            }
            _ => {}
        }
    }

    fn handle_window(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::SizeChanged(x, y) => {
                self.width = x as u32;
                self.height = y as u32;
            }
            _ => {}
        }
    }

    fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("mage", self.width, self.height)
            .resizable()
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let creator = canvas.texture_creator();
        let mut texture = creator.create_texture_static(
            Some(PixelFormatEnum::RGBA8888),
            self.width,
            self.height
        ).unwrap();
        self.image.fill(&mut texture).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();
        while (!self.should_end) {
            for event in event_pump.poll_iter() {
                self.handle(event);
            }
            let dest = Rect::new(0, 0, self.width, self.height);
            canvas.copy(&texture, None, dest);
            canvas.present();
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

pub fn display(image: Image) {
    Display::new(image).run();
}
