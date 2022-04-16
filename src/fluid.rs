

pub mod fluid {
    use glium::program::Uniform;
    use glium::{self, framebuffer::RenderBuffer};
    use glium::{glutin, Rect, BlitTarget, Display, Texture2d};
    use glium::glutin::{event,event_loop};
    use glium::Surface;
    //pub mod render;
    use crate::render::Render::{RenderObject, BufferManager};

    pub struct Fluid {
        pub resolution_x: f32,
        pub resolution_y: f32,
        pub viscosity: f32,
        pub time_step: f32,
        pub color: [f32;3],
        pub solve_itterations: u16,
        pub toggle_color_mode: bool,
        pub density_diffuse_rate: f32,
        pub velocity_diffuse_rate: f32,
        pub curl: f32,
        pub draw_size: f32,
        pub color_phase: f32,
        pub count: f32,
    }



    impl Fluid {

        pub fn new(x: f32, y: f32) -> Fluid {
            let resolution_x = x;
            let resolution_y = y;
            let viscosity = 1.0;
            let time_step = 0.3;
            let color = [1.0,1.0,1.0];
            let solve_itterations = 20;
            let toggle_color_mode = false;
            let density_diffuse_rate = 1.0;     // Technically no diffusion would a diffusion rate of 0.0, however this more analogous to a decay-multiplier - 1.0*previous value
            let velocity_diffuse_rate = 1.0;    // Technically no diffusion would a diffusion rate of 0.0, however this more analogous to a decay-multiplier - 1.0*previous value
            let curl = 0.0;
            let draw_size = 1.0;
            let color_phase = 0.0;
            let count = 0.0;

            Fluid {resolution_x,resolution_y,viscosity,time_step,color,solve_itterations,toggle_color_mode,density_diffuse_rate,velocity_diffuse_rate,curl,draw_size,color_phase,count}

        }


        
        pub fn initial(&mut self, render: &RenderObject, buffer: &mut glium::framebuffer::SimpleFrameBuffer,program: &glium::Program) {
            buffer.draw(&render.vertex,&render.indices,program,&uniform! {},&Default::default()).unwrap();
        }

        pub fn advect_velocity(&mut self,render: &RenderObject,program: &glium::Program ,target_texture: &mut glium::framebuffer::SimpleFrameBuffer,velocity_texture: &glium::Texture2d, mouse: [f32;4]) {
            
            let uniform = uniform! {
                texture1: glium::uniforms::Sampler(velocity_texture,render.behave),
                mouse: [mouse[0],mouse[1]],
                mouse_delta: [mouse[2],mouse[3]],
                res: [self.resolution_x,self.resolution_y],
                dt: self.time_step,
                enable: self.toggle_color_mode,
                rate: self.velocity_diffuse_rate,
            };

            target_texture.draw(&render.vertex,&render.indices,program,&uniform,&Default::default()).unwrap();
        }

        pub fn divergence(&mut self,render: &RenderObject,program: &glium::Program ,target_texture: &mut glium::framebuffer::SimpleFrameBuffer,velocity_texture: &glium::Texture2d) {
            
            let uniform = uniform! {
                texture2: glium::uniforms::Sampler(velocity_texture,render.behave),
                res: [self.resolution_x,self.resolution_y],
                dt: self.time_step,
            };

            target_texture.draw(&render.vertex,&render.indices,program,&uniform,&Default::default()).unwrap();
        }


        pub fn solve_pressure(&mut self,render: &RenderObject,program: &glium::Program ,target_texture1: &mut glium::framebuffer::SimpleFrameBuffer,swap_texture: &mut glium::framebuffer::SimpleFrameBuffer,buffers: &BufferManager) {

            let uniform1 = uniform! {
                texture1: glium::uniforms::Sampler(&buffers.buff_a,render.behave),
                texture4: glium::uniforms::Sampler(&buffers.buff_d,render.behave),
                first: true,
                res: [self.resolution_x,self.resolution_y],
                dt: self.time_step,
            };

            target_texture1.draw(&render.vertex,&render.indices,program,&uniform1,&Default::default()).unwrap();


            for n in 0..self.solve_itterations {
                let uniform2 = uniform! {
                    texture1: glium::uniforms::Sampler(&buffers.buff_a,render.behave),
                    texture4: glium::uniforms::Sampler(&buffers.temp,render.behave),
                    first: false,
                    res: [self.resolution_x,self.resolution_y],
                    dt: self.time_step,
                };
                
                swap_texture.draw(&render.vertex,&render.indices,program,&uniform2,&Default::default()).unwrap();
                let uniform1 = uniform! {
                    texture1: glium::uniforms::Sampler(&buffers.buff_a,render.behave),
                    texture4: glium::uniforms::Sampler(&buffers.buff_d,render.behave),
                    first: false,
                    res: [self.resolution_x,self.resolution_y],
                    dt: self.time_step,
                };
                target_texture1.draw(&render.vertex,&render.indices,program,&uniform1,&Default::default()).unwrap();      
                //std::mem::swap(&mut buffers.buff_a,&mut buffers.temp);
            }

        }

        pub fn project(&mut self,render: &RenderObject,program: &glium::Program ,target_texture: &mut glium::framebuffer::SimpleFrameBuffer,velocity_texture: &glium::Texture2d,pressure_texture: &glium::Texture2d) {
            
            let uniform = uniform! {
                texture2: glium::uniforms::Sampler(velocity_texture,render.behave),
                texture4: glium::uniforms::Sampler(pressure_texture,render.behave),
                res: [self.resolution_x,self.resolution_y],
                dt: self.time_step,
            };

            target_texture.draw(&render.vertex,&render.indices,program,&uniform,&Default::default()).unwrap();
        }


        pub fn advect_fluid(&mut self,render: &RenderObject,program: &glium::Program ,target_texture: &mut glium::framebuffer::SimpleFrameBuffer,velocity_texture: &glium::Texture2d,color_texture: &glium::Texture2d, mouse: [f32;4]) {

            let uniform = uniform! {
                texture1: glium::uniforms::Sampler(velocity_texture,render.behave),
                texture3: glium::uniforms::Sampler(color_texture,render.behave),
                mouse: [mouse[0],mouse[1]],
                mouse_delta: [mouse[2],mouse[3]],
                res: [self.resolution_x,self.resolution_y],
                dt: self.time_step,
                enable: self.toggle_color_mode,
                rate: self.density_diffuse_rate,
                count: self.count,
                colors: self.color,
                phase: self.color_phase,

            };

            target_texture.draw(&render.vertex,&render.indices,program,&uniform,&Default::default()).unwrap();

        }

        


        
    }

}