pub fn clear(red: f32, green: f32, blue: f32, alpha: f32, depth: bool) {
    let mut clear_flag = gl::COLOR_BUFFER_BIT;
    if depth {
        clear_flag |= gl::DEPTH_BUFFER_BIT;
    }
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
        gl::Clear(clear_flag);
    }
}

pub fn test_depth(test: bool) {
    if test {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    } else {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}
