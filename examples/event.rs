extern crate glutin;
extern crate scancode;

use std::collections::BTreeMap;
use scancode::Scancode;

fn main() {
    let mut errors = BTreeMap::new();
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().build(&events_loop).unwrap();
    let _ = unsafe { window.make_current() };
    // println!("Pixel format of the window: {:?}", window.get_pixel_format());
    // let context = support::load(&window);

    events_loop.run_forever(|event| {
        let _ = window.swap_buffers();

        let glutin::Event::WindowEvent { event, .. } = event;

        match event {
            glutin::WindowEvent::KeyboardInput(state, scancode, vk, _) => {
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
            glutin::WindowEvent::Closed => events_loop.interrupt(),
            _ => (),
        }
    });

    if !errors.is_empty() {
        println!("Unhandled scancodes:");
        for (k, v) in errors.iter() {
            println!("{}: {}", k, v);
        }
    }
}
