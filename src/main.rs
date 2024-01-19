// Imports :)
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{DeviceEvents, EventLoop},
    keyboard::Key,
    window::{WindowBuilder, WindowButtons, Window},
};
 
 // Main method, spawns the window
fn main() {
    // Necessary for logging within WGPU

    env_logger::init();

    // The event loop provided by winit.. for handling events
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("erosion")
        .build(&event_loop)
        .unwrap();
 
    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, elwt| {
        // Switching from general event type to window events
        if let Event::WindowEvent { window_id, event } = event {
            // Testing for different event types
            match event {
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key.as_ref() {
                    Key::Character("F" | "f") => {
                        let buttons = window.enabled_buttons();
                        window.set_enabled_buttons(buttons ^ WindowButtons::CLOSE);
                    }
                    Key::Character("G" | "g") => {
                        let buttons = window.enabled_buttons();
                        window.set_enabled_buttons(buttons ^ WindowButtons::MAXIMIZE);
                        window.set_maximized(true);
                        
                        // Checking if button is active and undoing operation if true
                        if buttons & WindowButtons::MAXIMIZE == buttons {
                            window.set_maximized(false);
                            window.set_enabled_buttons(buttons & (!WindowButtons::MAXIMIZE));
                        }
                    }
                    Key::Character("H" | "h") => {
                        let buttons = window.enabled_buttons();
                        window.set_enabled_buttons(buttons ^ WindowButtons::MINIMIZE);
                        window.set_minimized(true);

                        // Checking if button is active and undoing operation if true
                        if buttons & WindowButtons::MINIMIZE == buttons {
                            window.set_maximized(false);
                            window.set_enabled_buttons(buttons & (!WindowButtons::MINIMIZE));
                        }
                    }
                    _ => (), // doesn't match any of the given buttons, thrown
                },
                WindowEvent::CloseRequested if window_id == window.id() => elwt.exit(),
                _ => (),
            }
            // Getting the currently enabled buttons
            // let buttons = window.enabled_buttons();

            // // Checking if active and carrying out commands
            // if (buttons & WindowButtons::MAXIMIZE) == WindowButtons::MAXIMIZE {
            //     window.set_maximized(true);

            //     // Removing button from active buttons by creating bitmask of zeros, performing bitwise AND
            //     let buttons_removed = (!WindowButtons::MAXIMIZE) & buttons;
            //     window.set_enabled_buttons(buttons_removed);
            // }
            // else if (buttons & WindowButtons::MINIMIZE) == WindowButtons::MINIMIZE {
            //     window.set_minimized(true);

            //     // Removing button from active buttons by creating bitmask of zeros, performing bitwise AND
            //     let buttons_removed = (!WindowButtons::MINIMIZE) & buttons;
            //     window.set_enabled_buttons(buttons_removed);
            // }
        }
    })
    .expect("Error starting event flow");
}