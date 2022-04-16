#[macro_use]
extern crate glium;
extern crate glam;
use glium::{glutin, Rect, BlitTarget};
use glium::glutin::{event,event_loop};
use glium::Surface;

pub mod render;
use crate::render::Render::{Vertex};
pub mod shaders;

use eframe::egui;

pub mod fluid;
use crate::fluid::fluid::Fluid;

fn main() {
     
    let canvas = vec![Vertex {position: [-1.0,-1.0],texture: [0.0,0.0]},Vertex {position: [1.0,1.0],texture: [1.0,1.0]},Vertex {position: [1.0,-1.0],texture: [1.0,0.0]},Vertex {position: [-1.0,1.0],texture: [0.0,1.0]}];
    implement_vertex!(Vertex,position,texture);
    
    let sim_width = 400 as f32;
    let sim_height = sim_width;

    let width = 750.0;
    let height = 600.0;

    let events_loop = event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width,height))
        .with_title("Fluid Simulation: Version 12468463");

    let cb = glutin::ContextBuilder::new().with_srgb(false).with_depth_buffer(24);
    let mut display = glium::Display::new(window,cb,&events_loop).unwrap();
    let mut egui_glium = egui_glium::EguiGlium::new(&display);

    
    let vertex_buffer = glium::VertexBuffer::new(&display, &canvas).unwrap();
    let indices = glium::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList,&[0u16,1,3,0,1,2]).unwrap();
    
    // Lotta programs in this vector... idk what's best
    let programs = shaders::shaders::compile_programs(&display);
    let mut buffers = render::Render::BufferManager::new(&display,[sim_width as u32, sim_height as u32]);
    let render_window = render::Render::RenderObject::new(&display,vertex_buffer,indices,[sim_width,sim_height]);
    
    let mut mouse = render::Render::Mouse {x:0.0,y:0.0,dx:0.0,dy:0.0,last: [500.0,500.0]};

    let frame_time = std::time::Duration::from_nanos(16_666_667);
    let mut last = std::time::Instant::now();
    let mut moved = false;
    let mut fluid_instance = Fluid::new(sim_width,sim_height);


    // Set the initial velocity & color textures.
    let mut frame_buffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_c).unwrap();
    fluid_instance.initial(&render_window,&mut frame_buffer1, &programs[0]);
    
    let mut frame_buffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_a).unwrap();
    fluid_instance.initial(&render_window,&mut frame_buffer2, &programs[1]);
    


    events_loop.run(move |ev, _, control_flow| {

        *control_flow = glutin::event_loop::ControlFlow::Poll; //(next_frame_time);

        match &ev {
            event::Event::WindowEvent { event, .. } => {
                egui_glium.on_event(event);

                match event {event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                
                event::WindowEvent::CursorMoved {position,..} => {
                    mouse.x = sim_width/height*(position.x as f32)/1.25-sim_width/height*100.0*1.25;
                    mouse.y = render_window.resolution[1]-sim_width/height*(position.y as f32)/1.25;
                    fluid_instance.count += 0.001;
                    fluid_instance.count = fluid_instance.count%0.5;
                    moved = true;
                    return;

                },
                
                _ => return,
                }
            }

            event::Event::MainEventsCleared => {

                let now = std::time::Instant::now();
                let elapsed = now - last;

                if elapsed < frame_time {
                    return;
                }
                last = now;

                // Configure our buffer textures as framebuffers...
                buffers.buff_d = glium::texture::texture2d::Texture2d::empty_with_format(&display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, render_window.resolution[0] as u32, render_window.resolution[1] as u32).unwrap();
                let mut buffer_a = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_a).unwrap(); //B
                let mut buffer_b = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_b).unwrap(); //A
                let mut buffer_c = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.temp).unwrap(); //Temp
                let mut buffer_d = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.buff_d).unwrap();  //D
                

                // Slightly dodgy but oh well.
                if mouse.x!=mouse.last[0] || mouse.y!=mouse.last[1] && moved==true {
                    mouse.dx = mouse.last[0]  as f32 - mouse.x;
                    mouse.dy = mouse.last[1]  as f32 - mouse.y;
                    mouse.last = [mouse.x,mouse.y];
                    moved = false;
                }
                else {
                    // This is very lazy of me
                    mouse.x = -10.0;
                    mouse.y = -10.0;
                }



                fluid_instance.advect_velocity(&render_window, &programs[5], &mut buffer_b, &buffers.buff_a, [mouse.x,mouse.y,mouse.dx,mouse.dy]);
                fluid_instance.divergence(&render_window, &programs[6], &mut buffer_a, &buffers.buff_b);
                fluid_instance.solve_pressure(&render_window, &programs[7], &mut buffer_c, &mut buffer_d, &buffers);
                fluid_instance.project(&render_window, &programs[3], &mut buffer_a, &mut buffers.buff_b, &mut buffers.buff_d);
                fluid_instance.advect_fluid(&render_window, &programs[2], &mut buffer_c, &mut buffers.buff_a, &mut buffers.buff_c,[mouse.x,mouse.y,mouse.dx,mouse.dy]);



                let mut target = display.draw();

                target.blit_from_simple_framebuffer(&buffer_c,&Rect{
                    left: 0,bottom:0,width:render_window.resolution[0] as u32,height:render_window.resolution[1] as u32},
                    &BlitTarget{
                        left:125,bottom:0,width: ((width-100.0)*1.25) as i32 as i32,height: (height*1.25) as i32
                    }
                    , glium::uniforms::MagnifySamplerFilter::Linear,
                );


                let repaint = egui_glium.run(&display, |gui_ctx| {
                    // Do your egui code in here using gui_ctx to make windows

                    egui::SidePanel::left("my_side_panel").max_width(100.0).show(gui_ctx, |ui| {
                        ui.heading("Options");
                        if ui.button("Reset Fluid").clicked() {
                            let mut frame_buffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display, &buffers.temp).unwrap();
                            fluid_instance.initial(&render_window,&mut frame_buffer1, &programs[0]);
                        }
                        ui.label(format!("TIME PER FRAME: {}ms",elapsed.as_millis()));
                        ui.label(format!("FPS: {}",1.0/elapsed.as_secs_f32()));

                        ui.color_edit_button_rgb(&mut fluid_instance.color);
                        if ui.button("Enable Single Color").clicked() {
                            fluid_instance.toggle_color_mode = !fluid_instance.toggle_color_mode;
                        }
                        ui.label("Color Phase");
                        ui.add(egui::Slider::new(&mut fluid_instance.color_phase, 0.0..=10.0));
                        ui.label("Fade Rate");
                        ui.add(egui::Slider::new(&mut fluid_instance.density_diffuse_rate, 0.50..=1.0));
                        ui.label("Velocity Diffuse Rate");
                        ui.add(egui::Slider::new(&mut fluid_instance.velocity_diffuse_rate, 0.99..=1.0));
                        ui.label("Solve Itterations");
                        ui.add(egui::Slider::new(&mut fluid_instance.solve_itterations, 1..=60));

                    });
                    
                });

                egui_glium.paint(&mut display, &mut target);
                
                target.finish().unwrap();

                std::mem::swap(&mut buffers.buff_c,&mut buffers.temp); // Idk this is probably not the best solution lol

            },_ => (),
        }

    });
}
