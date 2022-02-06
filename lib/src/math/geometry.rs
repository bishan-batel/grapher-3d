use wasm_bindgen::{UnwrapThrowExt, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{
    parser::{lexer::Lexer, parser::Parser, ParseNode},
    render::glutils::{compile_shader, link_program},
};

// template
const FRAG_SHADER: &str = include_str!("../shaders/src/equation.frag");
const VERT_TEMPLATE: &str = include_str!("../shaders/src/equation.vert");

pub struct GraphEquation {
    pub ast: ParseNode,
    pub source: String, // TODO ammend to criterion B
    pub color: u8,
    pub program: WebGlProgram,
    pub old: ParseNode,
}

impl GraphEquation {
    pub fn new(
        gl: &WebGl2RenderingContext,
        eq: String,
        color: u8,
        old: Option<ParseNode>,
    ) -> Result<Self, JsValue> {
        // Parsing Text -------------------------------------------------------

        // tokenizes text
        let lexer = Lexer::new(eq.clone());
        let tokens = lexer.tokenize();

        // if tokenization returned error, stringify and return
        if let Err(err) = tokens {
            return Err(format!("{:?}", err).into());
        }

        // unwraps tokens if not an error
        let tokens = tokens.unwrap();

        // parses to abstract syntax tree
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        // if parsing to AST returned error, stringify and return
        if let Err(err) = ast {
            return Err(format!("{:?}", err).into());
        }

        let ast = ast.unwrap();
        Parser::validate(&ast)?;
        let ast_body = match &ast {
            ParseNode::FunctionDefine(_, _, body) => body,
            _ => panic!("Invalid State"),
        };

        crate::log(format!("Rendering: {}", ast_body).as_str());

        // unwraps old if exists, and if not set old to current
        let old_ast = if let Some(old) = old {
            // grabs body from old function AST
            match &old {
                ParseNode::FunctionDefine(_, _, body) => *body.clone(),
                _ => panic!("Invalid State"),
            }
        } else {
            *ast_body.clone()
        };

        // Shader Generation -------------------------------------------------
        let vert_shader = VERT_TEMPLATE
            .to_string()
            .replace("$CURRENT_FUNCTION$", format!("{}", ast_body).as_str())
            .replace("$OLD_FUNCTION$", format!("{}", old_ast).as_str())
            .replace("$EXTERN_FUNCTIONS$", "");

        // Setting up rendering program --------------------------------------

        // Compiles shaders
        let fragment_shader =
            compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, FRAG_SHADER)?;

        let vertex_shader = compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            vert_shader.as_str(),
        )?;

        // Binds fragment & vertex shader into the program
        // as well as links
        // Throws an error if there was any issues
        let program = link_program(&gl, &vertex_shader, &fragment_shader)?;

        gl.use_program(Some(&program));

        // asigns color
        if let Some(color_loc) = gl.get_uniform_location(&program, "graphColor") {
            // gets color theme from dom
            //
            let rgb_arr = crate::theme(11 + color as u32);

            // converts js values doubles to f32
            let r = rgb_arr.get(0).as_f64().unwrap_throw() as f32;
            let g = rgb_arr.get(1).as_f64().unwrap_throw() as f32;
            let b = rgb_arr.get(2).as_f64().unwrap_throw() as f32;

            // pases to shader
            gl.uniform4f(Some(&color_loc), r, g, b, 0.99);
        }

        Ok(Self {
            ast,
            program,
            source: eq,
            color,
            old: old_ast,
        })
    }
}
