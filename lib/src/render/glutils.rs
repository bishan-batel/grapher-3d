use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    src: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Unable to create shader")?;

    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    // If compile
    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error".into());

        Err(format!("Error compiling Shader: \n{}", err))
    }
}

pub fn link_program(
    gl: &WebGl2RenderingContext,
    vert: &WebGlShader,
    frag: &WebGlShader,
) -> Result<WebGlProgram, String> {
    // creates gl program
    let program = gl
        .create_program()
        .ok_or_else(|| "Failed to create program")?;

    // attaches compiled shaders
    gl.attach_shader(&program, vert);
    gl.attach_shader(&program, frag);

    // links program
    gl.link_program(&program);

    // get link status (or if failed to get the status unwrap it as false) and
    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        // return program if link status is true
        Ok(program)
    } else {
        // return error with the messsage from GL
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unkown error linking program".into()))
    }
}
