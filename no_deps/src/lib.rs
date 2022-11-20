use std::collections::HashMap;

// Import the GL functions that are used to make this demo work.
extern "C" {
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(bit_mask: u32);
}

/// Initialize a "game engine"
/// This function only gets called once.
#[no_mangle]
extern "C" fn init_engine() {
    let v = vec![0];
    unsafe { glViewport(v[0], 0, 280, 210) }
}

/// Pretend to update a game.
/// This gets called at the beginning of every animation frame.
#[no_mangle]
extern "C" fn update_game() {
    let a = vec![1, 0, 0, 1, 3];
    let mut b = 0;
    for v in a.iter() {
        b += v;
    }
}

/// Pretend to update graphics
/// This gets called at the end of every animation frame.
#[no_mangle]
extern "C" fn update_renderer() {
    let h = {
        let mut h = HashMap::new();
        h.insert("glColorBit", 0x00004000);
        h
    };
    let v = vec![0.0];
    unsafe {
        glClearColor(v[0], 1.0, 1.0, 1.0);
        glClear(*h.get("glColorBit").unwrap())
    }
}
