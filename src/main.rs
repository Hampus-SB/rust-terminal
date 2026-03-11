use glium;

fn main() {
    println!("Program start");

    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build().expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .build(&event_loop);
    
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
	        glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
	        _ => (),
            },
            _ => (),
        };
    });
    
    println!("Program end");
}






