use std::{thread, time::Duration};

use rand::Rng;
use sdl2::{pixels::Color, EventPump, event::Event, render::{Canvas, Texture}, video::Window, image::LoadTexture, rect::Rect};

const WIN_WIDTH: u32 = 640;
const WIN_HEIGHT: u32 = 480;
const IMG_WIDTH: u32 = 96;
const IMG_HEIGHT: u32 = 64;

struct Data {
    x: u32,
    y: u32,
    rx: bool,
    ry: bool,
}

fn run_event_loop(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => {}
        }
    }

    true
}

fn render(canvas: &mut Canvas<Window>, texture: &mut Texture, data: &mut Data) {
    canvas.clear();

    let mut rand = rand::thread_rng();

    data.x = if data.rx { data.x - 1 } else { data.x + 1 };
    data.y = if data.ry { data.y - 1 } else { data.y + 1 };

    if data.x + IMG_WIDTH >= WIN_WIDTH || data.x <= 0 {
        data.rx = !data.rx;
        texture.set_color_mod(rand.gen_range(0..=255), rand.gen_range(0..=255), rand.gen_range(0..=255));
    }
    if data.y + IMG_HEIGHT >= WIN_HEIGHT || data.y <= 0 {
        data.ry = !data.ry;
        texture.set_color_mod(rand.gen_range(0..=255), rand.gen_range(0..=255), rand.gen_range(0..=255));
    }

    canvas.copy(texture, None, Some(Rect::new(data.x as i32, data.y as i32, IMG_WIDTH, IMG_HEIGHT))).unwrap();
    canvas.present();
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem.window("DVD Bounce", WIN_WIDTH, WIN_HEIGHT)
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

    let mut data = Data {
        x: 0,
        y: 0,
        rx: false,
        ry: false,
    };

    loop {
        render(&mut canvas, &mut logo, &mut data);

        if !run_event_loop(&mut event_pump) {
            break;
        }

        thread::sleep(Duration::new(0, 1000000000 / 60));
    }
}
