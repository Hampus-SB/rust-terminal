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
    


    // Load the font
    let font_data = include_bytes!("/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Bold.ttf");
    // This only succeeds if collection consists of one font
    let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = rusttype::Scale::uniform(32.0);

    // The text to render
    let text = "This is RustType rendered into a png!";

    // Use a dark red colour
    let colour = (150, 0, 0);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Create a new rgba image with some padding
    let mut image = image::DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    image::Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }
    
    image.save("font.png").unwrap();



    let image = image::load(std::io::Cursor::new(&include_bytes!("../font.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();



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
