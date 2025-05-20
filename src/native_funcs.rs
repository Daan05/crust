use crate::value::StackValue;

pub fn clock(_args: &[StackValue]) -> StackValue {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    StackValue::F64(time.as_secs_f64())
}

pub fn print(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    print!("{}", string);

    StackValue::Null
}
pub fn println(args: &[StackValue]) -> StackValue {
    use colored::Colorize;

    let string = args[0].display().green();
    println!("{}", string);

    StackValue::Null
}

pub fn sin(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sin())
    } else {
        unreachable!()
    }
}

pub fn cos(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.cos())
    } else {
        unreachable!()
    }
}

pub fn tan(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.tan())
    } else {
        unreachable!()
    }
}

pub fn min(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.min(val2)),
        _ => unreachable!(),
    }
}

pub fn max(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.max(val2)),
        _ => unreachable!(),
    }
}

pub fn abs(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.abs())
    } else {
        unreachable!()
    }
}

pub fn sqrt(args: &[StackValue]) -> StackValue {
    let val = args[0];
    if let StackValue::F64(val) = val {
        StackValue::F64(val.sqrt())
    } else {
        unreachable!()
    }
}

pub fn pow(args: &[StackValue]) -> StackValue {
    let val1 = args[0];
    let val2 = args[1];
    match (val1, val2) {
        (StackValue::F64(val1), StackValue::F64(val2)) => StackValue::F64(val1.powf(val2)),
        _ => unreachable!(),
    }
}

/*
* This part of native functions is for sdl2 bindings
*
*
* And sdl2 state, like window
*
*/

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::cell::RefCell;

struct Sdl2State {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

thread_local! {
    static SDL_STATE: RefCell<Option<Sdl2State>> = RefCell::new(None);
}

pub fn sdl2_create_window(args: &[StackValue]) -> StackValue {
    let title = args[0];
    let width = args[1];
    let height = args[2];

    use sdl2::pixels::Color;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    if let (StackValue::F64(width), StackValue::F64(height)) = (width, height) {
        let window = video_subsystem
            .window(&title.display().to_string(), width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        SDL_STATE.with(|state| {
            *state.borrow_mut() = Some(Sdl2State { canvas, event_pump });
        });
    } else {
        unreachable!()
    }

    StackValue::Null
}

pub fn sdl2_handle_events(_args: &[StackValue]) -> StackValue {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    let mut quit = false;

    SDL_STATE.with(|state| {
        if let Some(ref mut sdl) = *state.borrow_mut() {
            for event in sdl.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => quit = true,
                    _ => {}
                }
            }
        }
    });

    StackValue::Bool(quit)
}

pub fn sdl2_clear_screen(_args: &[StackValue]) -> StackValue {
    use sdl2::pixels::Color;

    SDL_STATE.with(|state| {
        if let Some(ref mut sdl) = *state.borrow_mut() {
            sdl.canvas.set_draw_color(Color::RGB(64, 64, 64));
            sdl.canvas.clear();
        }
    });

    StackValue::Null
}

pub fn sdl2_update_screen(_args: &[StackValue]) -> StackValue {
    SDL_STATE.with(|state| {
        if let Some(ref mut sdl) = *state.borrow_mut() {
            sdl.canvas.present();
        }
    });

    StackValue::Null
}
