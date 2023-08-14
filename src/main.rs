const WIN_HEIGHT: u32 = 640;
const BAR_HEIGHT_MARGIN: u32 = 20;

use clap::Parser;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

use orst::*;

mod algorithms;
mod bar;
mod util;

use algorithms::*;
use bar::Bar;

fn draw_text(
    text: &str,
    c: &mut Canvas<Window>,
    tc: &TextureCreator<WindowContext>,
    font: &Font,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let text = font.render(text);
    let text_surface = text.blended(Color::BLACK).unwrap();
    let text_texture = text_surface.as_texture(tc).unwrap();

    c.copy(
        &text_texture,
        None,
        Rect::new(x, y, text_surface.width(), text_surface.height()),
    )
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "40")]
    bars: usize,
    #[arg(short, long, default_value = "20")]
    width: u32,
    #[arg(short, long, default_value = "gnome")]
    algorithm: AlgorithmType,
    #[arg(short, long, default_value = "60.0")]
    fps: f64,
}

fn main() {
    // BEFORE ANYTHING: INITIALISE CELLS
    let args = Args::parse();
    unsafe {
        NUM_ELEM_CELL.get_or_init(|| args.bars);
    }

    let num_elem_from_cell = unsafe { *NUM_ELEM_CELL.get().unwrap() };

    let mut list: List<Bar> = List::shuffled(num_elem_from_cell);
    let mut algorithm: Box<dyn Algorithm<Item = Bar>> = match args.algorithm {
        AlgorithmType::Gnome => Box::new(gnome::GnomeSort::new()),
        AlgorithmType::Bubble => Box::new(bubble::BubbleSort::new()),
        AlgorithmType::OptimisedBubble => Box::new(optimised_bubble::OptimisedBubbleSort::new()),
        AlgorithmType::Comb => Box::new(comb::CombSort::new()),
        AlgorithmType::Insertion => Box::new(insertion::InsertionSort::new()),
    };

    let sdl2_ctx = sdl2::init().expect("could not initialise SDL2.");
    let video_subsytem = sdl2_ctx
        .video()
        .expect("could not initialise the video subsystem.");
    let window = video_subsytem
        .window(
            "orst - a sorting algorithm visualiser",
            args.width * num_elem_from_cell as u32,
            WIN_HEIGHT,
        )
        .position_centered()
        .allow_highdpi()
        .borderless()
        .build()
        .expect("could not build the window.");
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_ctx = sdl2::ttf::init().unwrap();
    let font = font_ctx.load_font("assets/iosevka.ttf", 24).unwrap();

    let mut event_pump = sdl2_ctx.event_pump().unwrap();

    let mut goto_next = false;

    let title_text = font.render(algorithm.name());
    let title_text_surface = title_text.blended(Color::BLACK).unwrap();
    let title_text_texture = title_text_surface.as_texture(&texture_creator).unwrap();

    'run: loop {
        canvas.set_draw_color(Color::RGB(216, 222, 235));
        canvas.clear();

        match algorithm.tick(&mut list) {
            AlgorithmState::Busy => {}
            AlgorithmState::Done => {
                if goto_next {
                    algorithm.reset();
                    list = List::<Bar>::shuffled(num_elem_from_cell);
                    goto_next = false;
                }
            }
        }

        // display the list
        for i in 0..list.size {
            let bar = list.get(i);
            let colour = if bar.selected {
                Color::WHITE
            } else {
                bar.colour
            };

            let t = (bar.value + 1) as f64 / num_elem_from_cell as f64;
            let bar_height = ((1.0 - t) * BAR_HEIGHT_MARGIN as f64
                + t * (WIN_HEIGHT - BAR_HEIGHT_MARGIN) as f64) as u32;

            let rect = Rect::new(
                i as i32 * args.width as i32,
                (WIN_HEIGHT - bar_height) as i32,
                args.width,
                bar_height,
            );

            canvas.set_draw_color(colour);
            canvas.fill_rect(rect).unwrap();
        }
        canvas
            .copy(
                &title_text_texture,
                None,
                Rect::new(
                    10,
                    10,
                    title_text_surface.width(),
                    title_text_surface.height(),
                ),
            )
            .unwrap();
        draw_text(
            &format!("reads:  {}", list.reads),
            &mut canvas,
            &texture_creator,
            &font,
            10,
            34,
        )
        .unwrap();
        draw_text(
            &format!("writes: {}", list.writes),
            &mut canvas,
            &texture_creator,
            &font,
            10,
            58,
        )
        .unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'run,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    goto_next = true;
                }
                _ => {}
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_secs_f64(args.fps.recip()))
    }

    println!("thank you for using orst.");
}
