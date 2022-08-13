use rand::Rng;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
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

const PLAY_AREA_SIZE: (u32, u32) = (1000, 1000);
const WINDOW_SIZE: (u32, u32) = (1024, 1024);
const MARGIN: (u32, u32) = ((WINDOW_SIZE.0 - PLAY_AREA_SIZE.0)/2, (WINDOW_SIZE.1 - PLAY_AREA_SIZE.1)/2);
const SNAKE_SEGMENT_SIZE: u32 = 50;

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

    let mut snake = vec![
        Rect::new(
            MARGIN.0 as i32 + 100, MARGIN.1 as i32 + 100, SNAKE_SEGMENT_SIZE, SNAKE_SEGMENT_SIZE
        ),
    ];
    let play_area = Rect::new(
        MARGIN.0.try_into().unwrap(), MARGIN.1.try_into().unwrap(), PLAY_AREA_SIZE.0, PLAY_AREA_SIZE.1
    );
    let mut apple = spawn_apple(&mut rng);

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
        canvas.draw_rects(
            &snake
        )?;
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(
            play_area
        )?;
        canvas.fill_rect(
            apple
        )?;
        canvas.present();
        canvas.set_draw_color(background_color);

        if frame >= 15 {
            let appel_eaten = snake[0].contains_rect(apple);
            let next_snake = anim_snake_head(snake, dirs, appel_eaten);
            if next_snake.is_ok() {
                snake = next_snake.expect("Unr!");
            } else {
                break 'running;
            }
            if appel_eaten {
                apple = spawn_apple(&mut rng);
            }

            frame = 0;
        } else {
            frame += 1;
        }
    }

    Ok(())
}

fn anim_snake_head(mut prev_snake: Vec<Rect>, dirs: (i32, i32), apple_eaten: bool) -> Result<Vec<Rect>, ()> {
    // assert snake is moving
    let (dir_x, dir_y) = dirs;
    assert!(dir_x == -1 || dir_x == 0 || dir_x == 1);
    assert!(dir_y == -1 || dir_y == 0 || dir_y == 1);
    assert!((dir_x, dir_y) != (0, 0));
    println!("{:?}", prev_snake[0]);

    let head = calc_next_head(prev_snake.first().expect("Unr"), dir_x, dir_y);

    // check if snake is out of bounds
    if head.x >= (PLAY_AREA_SIZE.0 + MARGIN.0).try_into().unwrap() || head.x < MARGIN.0.try_into().unwrap() {
        return Err(())
    }

    if head.y >= (PLAY_AREA_SIZE.1 + MARGIN.1).try_into().unwrap() || head.y < MARGIN.1.try_into().unwrap() {
        return Err(())
    }

    // fill-in rest of the snake
    let next_snake = {
        let mut ns = Vec::with_capacity(prev_snake.len());
        ns.push(head);

        // if snake ate appel, last segment should "duplicate" itself
        if !apple_eaten {
            prev_snake.pop();
        }

        for prev_segment in prev_snake {
            ns.push(prev_segment)
        };
        ns
    };
    Ok(next_snake)
}

fn calc_next_head(current_segment: &Rect, dir_x: i32, dir_y: i32) -> Rect {
    Rect::new(
        current_segment.x() + (dir_x * current_segment.width() as i32),
        current_segment.y() + (dir_y * current_segment.width() as i32),
        current_segment.width(),
        current_segment.height()
    )
}

fn spawn_apple(rng: &mut ThreadRng) -> Rect {
    let x = SNAKE_SEGMENT_SIZE * rng.gen_range(0..PLAY_AREA_SIZE.0/SNAKE_SEGMENT_SIZE) + MARGIN.0;
    let y = SNAKE_SEGMENT_SIZE * rng.gen_range(0..PLAY_AREA_SIZE.1/SNAKE_SEGMENT_SIZE) + MARGIN.1;
    let rect = {
        let mut r = Rect::new(
            x as i32, y as i32, SNAKE_SEGMENT_SIZE/2, SNAKE_SEGMENT_SIZE/2
        );
        Rect::center_on(
            &mut r,
            Point::new((x + SNAKE_SEGMENT_SIZE/2) as i32, (y + SNAKE_SEGMENT_SIZE/2) as i32)
        );
        r
    };
    rect
}
