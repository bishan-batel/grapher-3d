// Native Functions
pub struct NativeFunc(pub &'static str, pub usize);

pub const NATIVE_FUNCTIONS: &[NativeFunc] = &[
    NativeFunc("time", 0),
    NativeFunc("sin", 1),
    NativeFunc("cos", 1),
    NativeFunc("tan", 1),
    NativeFunc("asin", 1),
    NativeFunc("acos", 1),
    NativeFunc("atan", 1),
    NativeFunc("mod", 2),
    NativeFunc("abs", 1),
    NativeFunc("fract", 1),
    NativeFunc("pow", 2),
    NativeFunc("exp", 1),
    NativeFunc("hypot", 2),
    NativeFunc("lerp", 3),
];


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
