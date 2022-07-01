use std::{thread, time::Duration};

use sdl2::{pixels::Color, EventPump, event::Event, render::{Canvas, Texture}, video::Window, image::LoadTexture};

fn run_event_loop(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => {}
        }
    }

    true
}

fn render(canvas: &mut Canvas<Window>, texture: &mut Texture) {
    canvas.clear();
    canvas.copy(texture, None, None).unwrap();
    canvas.present();
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem.window("DVD Bounce", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut logo = texture_creator.load_texture("dvd_logo.png").unwrap();

    loop {
        render(&mut canvas, &mut logo);

        if !run_event_loop(&mut event_pump) {
            break;
        }

        thread::sleep(Duration::new(0, 1000000000 / 60));
    }
}
