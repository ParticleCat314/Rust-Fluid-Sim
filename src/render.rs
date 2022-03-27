
pub mod Render {
    use glium::program::Uniform;
    use glium::{self, framebuffer::RenderBuffer};
    use glium::{glutin, Rect, BlitTarget, Display, Texture2d};
    use glium::glutin::{event,event_loop};
    use glium::Surface;

    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32;2],
        pub texture: [f32;2]
    }


    pub struct Mouse {
        pub x: f32,
        pub y: f32,
        pub dx: f32,
        pub dy: f32,
        pub last: [f32;2],
    }
        
    pub struct RenderObject {
        display: glium::Display,
        pub vertex: glium::vertex::VertexBuffer<Vertex>,
        pub indices: glium::IndexBuffer<u16>,
        behave: glium::uniforms::SamplerBehavior,
        pub resolution: [f32;2],
        dt: f32,
    }



    impl RenderObject {
        pub fn new(display: &glium::Display,vertex: glium::vertex::VertexBuffer<Vertex>,indices: glium::IndexBuffer<u16>, res: [f32;2],dt:f32) -> RenderObject {
            let behavior = glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::NearestMipmapLinear,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
                wrap_function: (glium::uniforms::SamplerWrapFunction::Repeat,glium::uniforms::SamplerWrapFunction::Repeat,glium::uniforms::SamplerWrapFunction::Repeat),
                //wrap_function: (glium::uniforms::SamplerWrapFunction::BorderClamp,glium::uniforms::SamplerWrapFunction::BorderClamp,glium::uniforms::SamplerWrapFunction::BorderClamp),
                ..Default::default()
            };

            RenderObject {display: display.clone(),vertex,indices,behave: behavior,resolution: res,dt}
        }




        pub fn initial(&mut self, buffer: &mut glium::framebuffer::SimpleFrameBuffer,program: &glium::Program) {
            buffer.draw(&self.vertex,&self.indices,program,&uniform! {},&Default::default()).unwrap();
        }


        pub fn draw2(&mut self, buffer: &mut glium::framebuffer::SimpleFrameBuffer,program: &glium::Program,buffers: &BufferManager,mouse: &Mouse) {
            let uniform = uniform! {
                texture1: glium::uniforms::Sampler(&buffers.buff_a,self.behave),
                texture2: glium::uniforms::Sampler(&buffers.buff_b,self.behave),
                texture3: glium::uniforms::Sampler(&buffers.buff_c,self.behave),
                texture4: glium::uniforms::Sampler(&buffers.buff_d,self.behave),
                //temp: glium::uniforms::Sampler(&buffers.temp,self.behave),
                mouse: [mouse.x,mouse.y],
                mouse_delta: [mouse.dx,mouse.dy],
                res: self.resolution,
                dt: self.dt,
            };

            //buffer.clear_color_srgb(0.0,0.0,0.0,0.0);

            buffer.draw(&self.vertex,&self.indices,program,&uniform,&Default::default()).unwrap();

        }
        pub fn jacobi_itteration(&mut self, program: &glium::Program,display: &glium::Display, buffers: &BufferManager,buffA: &mut glium::framebuffer::SimpleFrameBuffer,buffB: &mut glium::framebuffer::SimpleFrameBuffer) {

            let uniform1 = uniform! {
                texture1: glium::uniforms::Sampler(&buffers.buff_a,self.behave),
                texture4: glium::uniforms::Sampler(&buffers.buff_d,self.behave),
                first: true,
                res: self.resolution,
                dt: self.dt,
            };

            buffA.draw(&self.vertex,&self.indices,program,&uniform1,&Default::default()).unwrap();


            for n in 0..20 {
                let uniform2 = uniform! {
                    texture1: glium::uniforms::Sampler(&buffers.buff_a,self.behave),
                    texture4: glium::uniforms::Sampler(&buffers.temp,self.behave),
                    first: false,
                    res: self.resolution,
                    dt: self.dt,
                };
                
                buffB.draw(&self.vertex,&self.indices,program,&uniform2,&Default::default()).unwrap();
                let uniform1 = uniform! {
                    texture1: glium::uniforms::Sampler(&buffers.buff_a,self.behave),
                    texture4: glium::uniforms::Sampler(&buffers.buff_d,self.behave),
                    first: false,
                    res: self.resolution,
                    dt: self.dt,
                };
                buffA.draw(&self.vertex,&self.indices,program,&uniform1,&Default::default()).unwrap();      
                //std::mem::swap(&mut buffers.buff_a,&mut buffers.temp);
            }

        }

        }
        pub struct BufferManager {
            pub buff_a: glium::Texture2d,
            pub buff_b: glium::Texture2d,
            pub buff_c: glium::Texture2d,
            pub buff_d: glium::Texture2d,
            pub temp: glium::Texture2d,
        }
    
        impl BufferManager {
            pub fn new(display: &glium::Display,resolution: [u32;2]) -> BufferManager {
                
                let buff_a = glium::texture::texture2d::Texture2d::empty_with_format(display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, resolution[0], resolution[1]).unwrap();
                let buff_b = glium::texture::texture2d::Texture2d::empty_with_format(display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, resolution[0], resolution[1]).unwrap();
                let buff_c = glium::texture::texture2d::Texture2d::empty_with_format(display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, resolution[0], resolution[1]).unwrap();
                let buff_d = glium::texture::texture2d::Texture2d::empty_with_format(display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, resolution[0], resolution[1]).unwrap();
                let temp = glium::texture::texture2d::Texture2d::empty_with_format(display,glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap,  resolution[0], resolution[1]).unwrap();
                BufferManager {buff_a: buff_a,buff_b: buff_b,buff_c: buff_c,buff_d: buff_d, temp: temp}
            }
    }




}



// Stuff

    //let a = glium::program::ProgramCreationInput::SourceCode {
    //    vertex_shader: &vertex_code,
    //    tessellation_control_shader: None,
    //    tessellation_evaluation_shader: None,
    //    geometry_shader: None,
    //    fragment_shader: &fragment_code2,
    //    transform_feedback_varyings: None,
    //    outputs_srgb: false,
    //    uses_point_size: false,
    //};
//
    //let program9 = glium::Program::new(&display,a).unwrap();

