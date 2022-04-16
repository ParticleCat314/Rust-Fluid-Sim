

// Just load all the shaders into a vector.
pub mod shaders {

    pub fn compile_programs(display: &glium::Display) -> Vec<glium::Program> {
        let vertex_code = std::include_str!("shaders/vertex.glsl");

        let advect_code = std::include_str!("shaders/advect_color.glsl");
        let jacobi_code = std::include_str!("shaders/jacobi.glsl");
        let project_code = std::include_str!("shaders/project.glsl");
        let draw_code = std::include_str!("shaders/draw.glsl");
    
        let initial_scaler_code = std::include_str!("shaders/initial.glsl");
        let initial_velocity_code = std::include_str!("shaders/initialV.glsl");
        let advect_velocity_code = std::include_str!("shaders/advect_velocity.glsl");
        let divergence_code = std::include_str!("shaders/div.glsl");
        
        let initial_s = glium::Program::from_source(display, vertex_code, initial_scaler_code, None).unwrap();
        let initial_v = glium::Program::from_source(display, vertex_code, initial_velocity_code, None).unwrap();
    
    
        let advect_program = glium::Program::from_source(display, vertex_code, advect_code, None).unwrap();
        let project_program = glium::Program::from_source(display, vertex_code, project_code, None).unwrap();
        let draw_program = glium::Program::from_source(display, vertex_code, draw_code, None).unwrap();
        let advect_v_program = glium::Program::from_source(display, vertex_code, advect_velocity_code, None).unwrap();
        let divergence_program = glium::Program::from_source(display, vertex_code, divergence_code, None).unwrap();
        let jacobi_program = glium::Program::from_source(display, vertex_code, jacobi_code, None).unwrap();
    
        vec![initial_s,initial_v,advect_program,project_program,draw_program,advect_v_program,divergence_program,jacobi_program]
    
    }

}