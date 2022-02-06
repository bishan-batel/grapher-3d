/**
 * Testing Module, not for production
 */
use crate::math::mat4::{self, Mat4};
use crate::math::vec3::Vec3;
/// Module to contain unit tests for projectss
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::shaders::shader_generator::ShaderGenerator;

#[test]
pub fn mat4() {
    let mut mat = Mat4::IDENTITY.clone();
    mat.look_at(
        Vec3::new(0., 0., -8.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
    );

    /*
     * -1,0,0,0,
     * 0,1,0,0,
     * 0,0,-1,0,
     * 0,0,-8,1
     * */
    println!("{:?}", mat);
}

#[test]
pub fn parser() {
    println!("::Parser Test::");

    const INPUT_PARSE: &str = r#"
        f(x, y) = 2 * x + g(x)
    "#;

    println!("::Tokenizing::");
    let toks = Lexer::new(INPUT_PARSE.into()).tokenize();

    if let Err(err) = toks {
        println!("{:?}", err);
        return;
    }

    let toks = toks.unwrap();
    println!("{:?})", toks);

    println!("::Parsing::");
    let parser = Parser::new(toks).parse();

    if let Err(err) = parser {
        println!("{:?}", err);
        return;
    }

    let ast = parser.unwrap();
    println!("{}", ast);
}

#[test]
fn planegen() {
    let (points, indecies) = ShaderGenerator::generate_plane(3);

    println!("{:?}", points);
    println!("Length {:?}", points.len());

    println!();

    println!("{:?}", indecies);
    println!("Length {:?}", indecies.len());
}
