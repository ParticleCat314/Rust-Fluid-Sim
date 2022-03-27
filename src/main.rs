#[macro_use]
extern crate glium;
extern crate glam;
use glium::{glutin, Rect, BlitTarget};
use glium::glutin::{event,event_loop};
use glium::Surface;

pub mod render;
use crate::render::Render::{Vertex};
pub mod shaders;



fn main() {
     
    let canvas = vec![Vertex {position: [-1.0,-1.0],texture: [0.0,0.0]},Vertex {position: [1.0,1.0],texture: [1.0,1.0]},Vertex {position: [1.0,-1.0],texture: [1.0,0.0]},Vertex {position: [-1.0,1.0],texture: [0.0,1.0]}];
    implement_vertex!(Vertex,position,texture);
    

    let width = 400 as f32;
    let height = width;

    let events_loop = event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width,height))
        .with_title("Fluid Simulation: Version 1246846");

    let cb = glutin::ContextBuilder::new().with_srgb(false);
    let display = glium::Display::new(window,cb,&events_loop).unwrap();
    
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &canvas).unwrap();
    let indices = glium::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList,&[0u16,1,3,0,1,2]).unwrap();
    
    // Lotta programs in this vector... idk what's best
    let programs = shaders::shaders::compile_programs(&display);
    let mut buffers = render::Render::BufferManager::new(&display,[width as u32, height as u32]);
    let mut render_window = render::Render::RenderObject::new(&display,vertex_buffer,indices,[width,height],0.3);
    
    let mut mouse = render::Render::Mouse {x:0.0,y:0.0,dx:0.0,dy:0.0,last: [0.0,0.0]};


    // Write the initial scaler-field image into buffer C
    let mut frame_buffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_c).unwrap();
    render_window.initial(&mut frame_buffer1, &programs[0]);
    
    // Write the initial velocity-field into buffer A
    let mut frame_buffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_a).unwrap();
    render_window.initial(&mut frame_buffer2, &programs[1]);
    

    //std::thread::sleep(std::time::Duration::from_millis(500)); // Just added this as a sloppy way to pause before recording video.



    events_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
            
                event::WindowEvent::CursorMoved {position,..} => {
                    mouse.x = position.x as f32;
                    mouse.y = render_window.resolution[1]-position.y as f32;
                    return;
                }
                _ => return,

            }

            event::Event::MainEventsCleared => {
                // Configure our buffer textures as framebuffers...
                buffers.buff_d = glium::texture::texture2d::Texture2d::empty_with_format(&display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, render_window.resolution[0] as u32, render_window.resolution[1] as u32).unwrap();
                let mut buffer_a = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_a).unwrap(); //B
                let mut buffer_b = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_b).unwrap(); //A
                let mut buffer_c = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.temp).unwrap(); //Temp
                let mut buffer_d = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_d).unwrap();  //D
                

                // Slightly dodgy but oh well.
                if mouse.x!=mouse.last[0] || mouse.y!=mouse.last[1] {
                    mouse.dx = mouse.last[0]  as f32 - mouse.x;
                    mouse.dy = mouse.last[1]  as f32 - mouse.y;
                    mouse.last = [mouse.x,mouse.y];
                }



                // Yoooo this is where the fluid stuff happens! Sort of.
                // Advect velocity field from buffer c into buffer b
                render_window.draw2(&mut buffer_b, &programs[5], &buffers,&mouse);
                // Field gets re-written to buffer c in the projection stage
                // Buffer B is requried until the final projection step

                // Calculate divergence from buffer B into buffer A
                render_window.draw2(&mut buffer_a, &programs[6], &buffers,&mouse);
                // Buffer A is now required for the Jacobi Pressure Solver.

                
                // Jacobi
                render_window.jacobi_itteration(&programs[7],&display,& buffers,&mut buffer_c,&mut buffer_d);
                // Jacobi writes into both the buffer C & buffer D
                // Also needs access to buffer A for the divergence of the previous thingo.


                // Final Projection Step
                render_window.draw2(&mut buffer_a, &programs[3], &buffers,&mouse);
                // Buffer A is now free


                // Advect Scaler Field through the new velocity field.
                render_window.draw2(&mut buffer_c, &programs[2], &buffers,&mouse);


                // This way me playing around with different visualizations. Not really working.
                //render_window.draw2(&mut buffer_a,&programs[4],&buffers,&mouse);




                let target = display.draw();
                target.blit_from_simple_framebuffer(&buffer_c,&Rect{
                    left: 0,bottom:0,width:render_window.resolution[0] as u32,height:render_window.resolution[1] as u32},
                    &BlitTarget{
                        left:0,bottom:0,width:render_window.resolution[0] as i32,height:render_window.resolution[1] as i32
                    }
                    , glium::uniforms::MagnifySamplerFilter::Linear,
                );


                target.finish().unwrap();
                std::mem::swap(&mut buffers.buff_c,&mut buffers.temp); // Idk this is probably not the best solution lol

            },_ => (),
        }

    });
}
