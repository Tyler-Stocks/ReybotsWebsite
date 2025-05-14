use web_sys::window;

pub fn screen_width() -> u64 {
    window()
        .expect("Browser window doesn't exist?")
        .inner_width()
        .expect("Browser window doesn't have width?")
        .unchecked_into_f64() as u64 
}

pub fn height() -> u64 {
    window()
        .expect("Browser window doesn't exist?")
        .inner_height()
        .expect("Browser window doesn't have height?")
        .unchecked_into_f64() as u64 
}