
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
        pub behave: glium::uniforms::SamplerBehavior,
        pub resolution: [f32;2],
    }



    impl RenderObject {
        pub fn new(display: &glium::Display,vertex: glium::vertex::VertexBuffer<Vertex>,indices: glium::IndexBuffer<u16>, res: [f32;2]) -> RenderObject {
            let behavior = glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::NearestMipmapLinear,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
                wrap_function: (glium::uniforms::SamplerWrapFunction::Repeat,glium::uniforms::SamplerWrapFunction::Repeat,glium::uniforms::SamplerWrapFunction::Repeat),
                //wrap_function: (glium::uniforms::SamplerWrapFunction::BorderClamp,glium::uniforms::SamplerWrapFunction::BorderClamp,glium::uniforms::SamplerWrapFunction::BorderClamp),
                ..Default::default()
            };

            RenderObject {display: display.clone(),vertex,indices,behave: behavior,resolution: res}
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

