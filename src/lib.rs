use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{self},
};

pub fn run() ->  Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

        let window = window::WindowBuilder::new()
            .with_title("erosion: the swarm simulator")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 800.0))
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, elwt| {
            // println!("{event:?}");
            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        // Notify the windowing system that we'll be presenting to the window.
                        window.pre_present_notify();
                        //fill::fill_window(&window);
                    }
                    _ => (),
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
}