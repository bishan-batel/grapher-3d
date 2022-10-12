pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global window defined")
}