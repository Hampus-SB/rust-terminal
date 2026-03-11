#[macro_use]
extern crate glium;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn get_vertex_shader() -> &'static str {
    return r#"
        #version 330 core

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
}

fn get_fragment_shader() -> &'static str {
    return r#"
        #version 330 core
        
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
}

fn main() {
    println!("Program start");

    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .build(&event_loop);

    let vertex_shader = get_vertex_shader();
    let fragment_shader = get_fragment_shader();

    let program = glium::Program::from_source(
        &display,
        vertex_shader, fragment_shader,
        None).unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
	        glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // draw code
                    let triangle = vec![
                        Vertex { position: [-1.0, -1.0] },
                        Vertex { position: [0.0, 1.0] },
                        Vertex { position: [1.0, -1.0] }
                    ];

                    let vertex_buffer = glium::VertexBuffer::new(
                        &display, &triangle).unwrap();

                    let index_buffer = glium::index::NoIndices(
                        glium::index::PrimitiveType::TrianglesList);

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    target.draw(&vertex_buffer, &index_buffer, &program, 
                        &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
                    target.finish().unwrap();
                },
	        _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });

    println!("Program end");
}
