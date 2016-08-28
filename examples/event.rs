extern crate glutin;
extern crate scancode;

use std::collections::BTreeMap;
use scancode::Scancode;

fn main() {
    let mut errors = BTreeMap::new();
    let window = glutin::WindowBuilder::new().build().unwrap();
    let _ = unsafe { window.make_current() };
    // println!("Pixel format of the window: {:?}", window.get_pixel_format());
    // let context = support::load(&window);

    for event in window.wait_events() {
        // context.draw_frame((0.0, 1.0, 0.0, 1.0));
        let _ = window.swap_buffers();

        match event {
            glutin::Event::KeyboardInput(state, scancode, vk) => {
                if state == glutin::ElementState::Pressed {
                    print!("pressed")
                } else {
                    print!("released")
                }

                print!(" {} -> ", scancode);
                if let Some(code) = Scancode::new(scancode) {
                    print!("{:?}", code);
                } else {
                    print!("*** ERROR: UNKNOWN SCANCODE ***");
                    errors.insert(scancode, format!("{:?}", vk));
                }

                if let Some(vk) = vk {
                    println!(" (virtual keycode {:?})", vk);
                } else {
                    println!(" (virtual keycode UNKNOWN)");
                }
            }
            glutin::Event::Closed => break,
            _ => (),
        }
    }

    if !errors.is_empty() {
        println!("Unhandled scancodes:");
        for (k, v) in errors.iter() {
            println!("{}: {}", k, v);
        }
    }
}
