use crate::sdl2::pixels::{Color, PixelFormatEnum};
use crate::sdl2::event::Event;
use crate::sdl2::keyboard::Keycode;

use crate::image::Image;


pub fn display(image: Image) {
    let width = image.width as u32;
    let height = image.height as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("mage", width, height)
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
        width,
        height
    ).unwrap();
    image.fill(&mut texture).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.present();
   }
}
