// Native Functions 

pub const NATIVE_CONSTANTS: &[NativeConstant] = &[
    NativeConstant("pi", std::f32::consts::PI),
    NativeConstant("tau", std::f32::consts::TAU),
    NativeConstant("e", std::f32::consts::E),
];

pub const NATIVE_VARS: &[&'static str] = &[
    "x",
    "y",
    "TIME",
    "t"
];

pub const NATIVE_FUNCTIONS: &[NativeFunc] = &[
    NativeFunc("time", 0),
    NativeFunc("sin", 1),
    NativeFunc("cos", 1),
    NativeFunc("tan", 1),
    NativeFunc("asin", 1),
    NativeFunc("acos", 1),
    NativeFunc("atan", 1),
    NativeFunc("round", 1),
    NativeFunc("floor", 1),
    NativeFunc("ceil", 1), 
    NativeFunc("mod", 2),
    NativeFunc("abs", 1),
    NativeFunc("fract", 1),
    NativeFunc("pow", 2),
    NativeFunc("exp", 1),
    NativeFunc("sqrt", 1),
    NativeFunc("hypot", 2),
    NativeFunc("lerp", 3),
];

pub struct NativeConstant(pub &'static str, pub f32); 

impl NativeConstant {}

pub struct NativeFunc(pub &'static str, pub usize);
impl NativeFunc {
    pub fn is_native(func: &(String, usize)) -> bool {
        // checks for matches with list of native functions
        for native in NATIVE_FUNCTIONS.iter() {
            let same_name = native.0 == func.0;
            let same_arg_count = native.1 == func.1;

            if same_name && same_arg_count {
                return true;
            }
        }

        false
    }
}
