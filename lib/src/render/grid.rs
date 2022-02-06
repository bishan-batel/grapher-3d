use js_sys::{Float32Array, Uint16Array, Uint32Array};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};

use crate::{
    math::mat4::{Mat4, Transform},
    shaders::shader_generator::ShaderGenerator,
};

use super::glutils::{compile_shader, link_program};

const GRID_FRAG_SHADER: &str = include_str!("../shaders/src/grid.frag");
const GRID_VERT_SHADER: &str = include_str!("../shaders/src/grid.vert");

pub struct Grid {
    program: WebGlProgram,
    pub freq: f32,
    pub vao: WebGlVertexArrayObject,
    pub index_len: usize,
    pub transform: Transform,
    //pub color: (f32, f32, f32, f32),
}

impl Grid {
    pub fn new(gl: &WebGl2RenderingContext, freq: f32) -> Result<Self, JsValue> {
        // compiles shaders
        let frag_shader = compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            GRID_FRAG_SHADER,
        )?;

        let vert_shader =
            compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, GRID_VERT_SHADER)?;

        // links program
        let program = link_program(gl, &vert_shader, &frag_shader)?;

        gl.use_program(Some(&program));

        // Creates & Binds VAO
        let vao = gl.create_vertex_array().unwrap_throw();
        gl.bind_vertex_array(Some(&vao));

        let (verticies, indecies) = ShaderGenerator::generate_plane(2);
        let verticies = verticies.as_slice();
        let indecies = indecies.as_slice();

        // Creates vertex buffer
        let vertex_buffer = gl.create_buffer().unwrap_throw();
        {
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

            // fills data into buffer
            unsafe {
                let view = Float32Array::view(verticies);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            }
        }

        // Creates index buffer
        let index_buffer = gl.create_buffer().unwrap_throw();
        {
            gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&index_buffer),
            );

            unsafe {
                let view = Uint32Array::view(&indecies);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                    &view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            }
        }

        // enables attribute array
        gl.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            3 * std::mem::size_of::<f32>() as i32,
            0,
        );
        //gl.vertex_attrib_pointer_with_i32(
        //1,
        //3,
        //WebGl2RenderingContext::FLOAT,
        //false,
        //6 * std::mem::size_of::<f32>() as i32,
        //3 * std::mem::size_of::<f32>() as i32,
        //);
        gl.enable_vertex_attrib_array(0);
        //gl.enable_vertex_attrib_array(1);

        // unbinds vao
        gl.bind_vertex_array(None);

        let out = Self {
            program,
            transform: Transform {
                proj: Mat4::new(),
                view: Mat4::new(),
                model: Mat4::new(),
            },
            vao,
            index_len: indecies.len(),
            //color: (0.5, 0.5, 0.5, 1.),
            freq,
        };

        out.set_uniforms(gl);

        Ok(out)
    }

    pub fn render(&self, gl: &WebGl2RenderingContext) {
        // binds VAO for vertex info
        gl.bind_vertex_array(Some(&self.vao));

        // assign program from gl to use
        gl.use_program(Some(&self.program));

        // issues drawcall
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.index_len as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );

        // unbinds vao
        gl.bind_vertex_array(None);
    }

    pub fn set_uniforms(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));

        let loc = |name: &str| gl.get_uniform_location(&self.program, name);

        if let Some(loc) = gl.get_uniform_location(&self.program, "mProj") {
            gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &self.transform.proj.0);
        }

        if let Some(loc) = gl.get_uniform_location(&self.program, "mView") {
            gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &self.transform.view.0);
        }

        if let Some(loc) = gl.get_uniform_location(&self.program, "mModel") {
            gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &self.transform.model.0);
        }

        if let Some(loc) = loc("frequency") {
            gl.uniform1f(Some(&loc), self.freq);
        }
    }
}
