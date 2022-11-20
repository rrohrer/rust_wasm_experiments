use std::collections::HashMap;

extern "C" {
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(bit_mask: u32);
}

#[no_mangle]
extern "C" fn init_engine() {
    let v = vec![0];
    unsafe { glViewport(v[0], 0, 280, 210) }
}

#[no_mangle]
extern "C" fn update_game() {}

#[no_mangle]
extern "C" fn update_renderer() {
    let mut h = HashMap::new();
    h.insert("glColorBit", 0x00004000);
    let v = vec![0.0];
    unsafe {
        glClearColor(v[0], 1.0, 1.0, 1.0);
        glClear(*h.get("glColorBit").unwrap())
    }
}
