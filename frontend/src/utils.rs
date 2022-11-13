use web_sys::window;

pub fn is_mobile() -> bool {
    let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
    let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

    width <= 600. && height <= 960.
}