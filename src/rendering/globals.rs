pub fn clear(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
