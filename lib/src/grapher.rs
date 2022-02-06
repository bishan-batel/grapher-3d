/*
 * This file holds the Grapher structure, which is a Singleton class that is used
 * to manage the state of the graph on the webpage, it is required for effieicny because
 * crearting things like WebGLPrograms & Buffers each frame would not be the most efficient
 * solution
 */

use js_sys::{Array, Date, Float32Array, Uint32Array};
use wasm_bindgen::{prelude::*, throw_str, JsCast};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject,
};

use crate::{
    math::{
        geometry::GraphEquation,
        mat4::{Mat4, Transform},
        vec3::Vec3,
    },
    render::{
        glutils::{compile_shader, link_program},
        grid::Grid,
    },
    shaders::shader_generator::ShaderGenerator,
};

// ----------------------------------------------------------------------------
// DEBUG CONSTANTS
// ----------------------------------------------------------------------------
const DRAW_WIRE: bool = false;
const DETAIL: usize = 300;

const ANIM_SPEED: f32 = 0.03;
const CAM_ZOOM_OUT: f32 = 1.5;

// ----------------------------------------------------------------------------
// Structure Definition
// ----------------------------------------------------------------------------

#[wasm_bindgen(js_name = GlobalGrapher)]
pub struct Grapher {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    equations: Vec<GraphEquation>,
    equation_plane_vao: Option<WebGlVertexArrayObject>,
    plane_index_length: u32,
    old_to_new: f32,
    cam: Vec3,
    animate: bool,
    grid: Grid,
}

// ----------------------------------------------------------------------------
// Grapher Implementation
// ----------------------------------------------------------------------------
#[wasm_bindgen(js_class = GlobalGrapher)]
impl Grapher {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        crate::log("Initializing Global");

        // grabs web gl context
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>();

        // if context does not exist then web GL unsupported
        if gl.is_err() {
            throw_str("WebGL Unsupported, Please update your browser");
        }

        let gl = gl.unwrap();

        let grid = Grid::new(&gl, 40.).unwrap();

        Self {
            // gets context from canvas and dynamically converts to correct type
            canvas,
            gl,
            equation_plane_vao: None,
            equations: vec![],
            plane_index_length: 0,
            cam: Vec3::new(0., 0., 8.),
            old_to_new: 0.,
            animate: true,
            grid,
        }
    }

    // initializes graph
    pub fn init(&mut self) -> Result<(), JsValue> {
        // ---------------------------------------------------------------------
        // Enable Important WebGL Features ------------------------------------
        // ---------------------------------------------------------------------

        // enable depth test, which prevents an object behind
        // another being rendered in front
        self.gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        // enable blend, which allows for alpha color blending
        self.gl.enable(WebGl2RenderingContext::BLEND);
        self.gl.blend_func(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        // enables cull face
        //self.gl.enable(WebGl2RenderingContext::CULL_FACE);
        //self.gl.front_face(WebGl2RenderingContext::CCW);
        //self.gl.cull_face(WebGl2RenderingContext::BACK);

        self.gl.get_extension("OES_element_index_uint").unwrap();

        // set clear color
        self.gl.clear_color(46. / 255., 52. / 255., 64. / 255., 1.);

        // ---------------------------------------------------------------------
        // Creates plane mesh which is used for all graphs
        // ---------------------------------------------------------------------

        // Create & bind VAO to record vert info
        let vao = self.gl.create_vertex_array().unwrap_throw();
        self.gl.bind_vertex_array(Some(&vao));

        // stores VAO to use for every frame
        self.equation_plane_vao = Some(vao);

        let (verticies, indecies) = ShaderGenerator::generate_plane(DETAIL);
        let verticies = verticies.as_slice();
        let indecies = indecies.as_slice();

        self.plane_index_length = indecies.len() as u32;

        // Setup for vertex buffer --------------------------------------------
        let vertex_buff = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        // binds created buffer
        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buff));

        // fills vertex buff
        unsafe {
            // allocate view to put verticies in & fills in with vertex data
            let vert_view = Float32Array::view(verticies);
            // fill buffer with data
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        // --------------------------------------------------------------------

        // Setup for vertex indices buffer ------------------------------------
        let index_buff = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buff),
        );

        // fill data into index view
        unsafe {
            let index_view = Uint32Array::view(indecies);

            // fill view data into index buffer
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        // --------------------------------------------------------------------

        // Setup shader vertex attributes
        self.gl.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            3 * std::mem::size_of::<f32>() as i32,
            0,
        );
        self.gl.enable_vertex_attrib_array(0);

        // unbind VAO
        self.gl.bind_vertex_array(None);

        // Clears Screen
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        Ok(())
    }

    pub fn render(&mut self) {
        // Clears screen (color & depth screen buffer)
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        // camera position to use for lookAt matrix & lighting calculations
        // inside fragment shader
        let cam_pos = Vec3::new(0., 0., -CAM_ZOOM_OUT);

        // Matrix Calculations ------------------------------------------------

        // World matrix for all graphs
        let mut m_world = Mat4::IDENTITY.clone();
        m_world.rotate_x(90f32.to_radians());
        m_world.rotate_z(self.cam.1);
        m_world.rotate_x(self.cam.0);

        // View matrix for all graphs
        let m_view = {
            let mut m_view = Mat4::new();
            m_view.look_at(cam_pos, Vec3::new(0., 0., 0.), Vec3::UP);
            m_view
        };

        // Projection matrix for all graphs
        let m_proj = {
            let mut m_proj = Mat4::new();
            // temp values
            let client_width = self.canvas.client_width() as f32;
            let client_height = self.canvas.client_height() as f32;

            m_proj.perspective(
                (45f32).to_radians(),
                client_width / client_height,
                0.01,
                99999.,
            );
            m_proj
        };

        // getting time value as seconds
        let time = {
            // gets date object
            let utc_millis = Date::now();
            let date = Date::new(&JsValue::from_f64(utc_millis));

            // double cast to work around issues with rounding errors due to
            // floating point precision problems when scaling from 64bits to 32
            let rounded_time = date.get_time() as f32 as f64;
            let millis = (date.get_time() - rounded_time) as f32;

            // converts to seconds
            millis / 1000.
        };

        // updates old to new & clamps value
        if self.animate {
            self.old_to_new = (self.old_to_new + ANIM_SPEED).clamp(0., 1.);
        } else {
            self.old_to_new = 1.;
        }

        self.grid.freq = self.cam.2;

        // --------------------------------------------------------------------
        // Rendering of each equation
        // --------------------------------------------------------------------

        // binds VAO
        let vao = self.equation_plane_vao.as_ref().unwrap_throw();
        self.gl.bind_vertex_array(Some(vao));

        let uni_loc =
            |eq: &GraphEquation, name: &str| self.gl.get_uniform_location(&eq.program, name);

        for equation in self.equations.iter() {
            // tell gl state machine to use the equation program
            self.gl.use_program(Some(&equation.program));

            // Grabbing uniform locations -------------------------------------
            // matrix locations
            let world_loc = uni_loc(&equation, "mWorld").unwrap();
            let view_loc = uni_loc(&equation, "mView").unwrap();
            let proj_loc = uni_loc(&equation, "mProj").unwrap();

            // Setting matrix uniforms
            self.gl
                .uniform_matrix4fv_with_f32_array(Some(&world_loc), false, &m_world.0);
            self.gl
                .uniform_matrix4fv_with_f32_array(Some(&proj_loc), false, &m_proj.0);
            self.gl
                .uniform_matrix4fv_with_f32_array(Some(&view_loc), false, &m_view.0);

            if let Some(graph_freq_loc) = uni_loc(&equation, "graphFrequency") {
                self.gl.uniform1f(Some(&graph_freq_loc), self.grid.freq);
            }

            // setting lighting position
            let light_pos_loc = uni_loc(&equation, "globalLightPosition");
            if let Some(light_pos_loc) = light_pos_loc {
                self.gl.uniform3f(Some(&light_pos_loc), 10., 10., 10.);
            }

            // Setting optional uniforms if they exist
            if let Some(time_loc) = uni_loc(&equation, "TIME") {
                self.gl.uniform1f(Some(&time_loc), time);
            }

            // Setting optional uniforms if they exist
            if let Some(old_to_new_loc) = uni_loc(&equation, "oldToNew") {
                self.gl.uniform1f(Some(&old_to_new_loc), self.old_to_new);
            }

            // Draw Call ------------------------------------------------------
            // draws program triangles (wiremesh of DRAW_WIRE debug constant is on)
            self.gl.draw_elements_with_i32(
                if DRAW_WIRE {
                    WebGl2RenderingContext::LINES
                } else {
                    WebGl2RenderingContext::TRIANGLES
                },
                self.plane_index_length as i32,
                WebGl2RenderingContext::UNSIGNED_INT,
                0,
            );
        }

        // unbinds VAO
        self.gl.bind_vertex_array(None);

        // --------------------------------------------------------------------
        // Renders Grid
        // --------------------------------------------------------------------
        self.grid.transform = Transform {
            proj: m_proj,
            view: m_view,
            model: m_world,
        };

        self.grid.set_uniforms(&self.gl);
        self.grid.render(&self.gl);
    }

    // ------------------------------------------------------------------------
    // Setters & Getters
    // ------------------------------------------------------------------------
    pub fn canvas(&self) -> HtmlCanvasElement {
        self.canvas.clone()
    }

    /// Sets graph equations through a JS Array
    /// Each element of the array represeents a graph
    /// Each element is an array with 2 values: the first value is
    /// whether or not the array is disabled, and the 2nd value is
    /// the actual graph equations  (eg. f(x)=2*x)
    #[wasm_bindgen]
    pub fn set_equations(&mut self, equations: Array) -> Result<(), JsValue> {
        // empties equation vectors

        let mut new_equations = vec![];

        // enumerates for every element in array
        for (i, js_equation) in equations.iter().enumerate() {
            // cast element to JS Array
            let js_equation: Array = js_equation.into();

            // skips if disabled
            let disabled = js_equation.get(0);
            if disabled.as_bool().unwrap_throw() {
                continue;
            }

            let ascii_js = js_equation.get(1);

            // converts to rust String
            let ascii = ascii_js.as_string().unwrap();

            // attempts to get the old graph for animation
            let old = if let Some(old_eq) = self.equations.iter().nth(i) {
                Some(old_eq.ast.clone())
            } else {
                None
            };

            // attempts to create graph equation
            let equation = GraphEquation::new(&self.gl, ascii, i as u8 % 5, old);

            // return error in failure, add to equations list otherwise
            if let Err(err) = equation {
                return Err(format!(
                    "Error on equation {}\n{}",
                    i + 1,                    // eq number
                    err.as_string().unwrap()  // msg
                )
                .into());
            } else if let Ok(equation) = equation {
                new_equations.push(equation)
            }
        }

        // clears equations
        for eq in self.equations.iter() {
            self.gl.delete_program(Some(&eq.program));
        }
        self.equations.clear();

        self.equations = new_equations;
        self.old_to_new = 0.;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_viewport(&self) {
        self.gl.viewport(
            0,
            0,
            self.canvas.client_width() as i32,
            self.canvas.client_height() as i32,
        );
    }

    #[wasm_bindgen]
    pub fn set_cam_rot(&mut self, x: f32, y: f32) {
        self.cam.0 = x;
        self.cam.1 = y;
    }

    #[wasm_bindgen]
    pub fn set_cam_zoom(&mut self, zoom: f32) {
        self.cam.2 = zoom;
    }

    #[wasm_bindgen]
    pub fn set_animate(&mut self, v: bool) {
        self.animate = v;
    }

    #[wasm_bindgen]
    pub fn cam_rot_x(&self) -> f32 {
        self.cam.0
    }

    #[wasm_bindgen]
    pub fn cam_rot_y(&self) -> f32 {
        self.cam.1
    }

    #[wasm_bindgen]
    pub fn cam_zoom(&self) -> f32 {
        self.cam.2
    }
}
