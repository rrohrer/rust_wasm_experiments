extern "C" {
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(bit_mask: u32);
}

#[no_mangle]
extern "C" fn init_engine() {
    unsafe { glViewport(0, 0, 280, 210) }
}

#[no_mangle]
extern "C" fn update_game() {}

#[no_mangle]
extern "C" fn update_renderer() {
    unsafe {
        glClearColor(0.0, 1.0, 1.0, 1.0);
        glClear(0x00004000)
    }
}
