use rand::Rng;
use rand::prelude::SliceRandom;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;


fn main() -> Result<(), String>{
    println!("Hello, world!");
    main_loop()
}

struct SnakeNode {
    x: u32,
    y: u32,
}

const DIMENSION: u32 = 20;
const PLAY_AREA_SIZE: (u32, u32) = (1000, 1000);
const WINDOW_SIZE: (u32, u32) = (1024, 1024);
const MARGIN: (u32, u32) = ((WINDOW_SIZE.0 - PLAY_AREA_SIZE.0)/2, (WINDOW_SIZE.1 - PLAY_AREA_SIZE.1)/2);

pub fn main_loop() -> Result<(), String> {
    let (init_size_x, init_size_y) = (1024, 1024);
    let init_name = "Visuals";
    let mut rng = rand::thread_rng();
    let background_color = Color::BLACK;
    let sdl_context = sdl2::init()?;

    let mut canvas = {
        let video = sdl_context.video()?;
        let window = video.window(init_name, init_size_x, init_size_y)
            .position_centered()
            // .resizable()
            .build()
            .expect("Failed to create window");
        window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Failed to get render canvas")
    };
    let mut events = sdl_context.event_pump()?;
    let (x_size, y_size) = canvas.output_size()?;

    // let cell_size = x_size / DIMENSION;

    let mut snake = Rect::new(
        MARGIN.0.try_into().unwrap(), MARGIN.1.try_into().unwrap(), 50, 50
    );
    let play_area = Rect::new(
        MARGIN.0.try_into().unwrap(), MARGIN.1.try_into().unwrap(), PLAY_AREA_SIZE.0, PLAY_AREA_SIZE.1
    );

    let mut dirs = (1, 0);
    let mut frame = 0;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    dirs = (-1, 0)
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    dirs = (1, 0)
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    dirs = (0, -1)
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    dirs = (0, 1)
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_rect(
            snake
        )?;
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(
            play_area
        )?;
        canvas.present();
        canvas.set_draw_color(background_color);


        if frame >= 15 {

            let next_snake = anim_snake_head(&snake, dirs.0, dirs.1);
            if next_snake.is_ok() {
                snake = next_snake.expect("Unr!");
            } else {
                break 'running;
            }

            frame = 0;
        } else {
            frame += 1;
        }
    }

    Ok(())
}

fn anim_snake_head(snake_head: &Rect, dir_x: i32, dir_y: i32) -> Result<Rect, ()> {
    assert!(dir_x == -1 || dir_x == 0 || dir_x == 1);
    assert!(dir_y == -1 || dir_y == 0 || dir_y == 1);
    assert!((dir_x, dir_y) != (0, 0));
    println!("{:?}", snake_head);
    let rect = Rect::new(
        snake_head.x() + (dir_x * snake_head.width() as i32),
        snake_head.y() + (dir_y * snake_head.width() as i32),
        snake_head.width(),
        snake_head.height()
    );

    if rect.x >= (PLAY_AREA_SIZE.0 + MARGIN.0).try_into().unwrap() || rect.x < MARGIN.0.try_into().unwrap() {
        return Err(())
    }

    if rect.y >= (PLAY_AREA_SIZE.1 + MARGIN.1).try_into().unwrap() || rect.y < MARGIN.1.try_into().unwrap() {
        return Err(())
    }
    Ok(rect)
}
