use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Position {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
pub struct SnakeGame {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> SnakeGame {
        SnakeGame { width, height }
    }

    #[wasm_bindgen(getter)]
    pub fn cherry_positions(&self) -> *const u32 {
        let mut result = Vec::new();

        result.push(10);
        result.push(10);

        result.as_ptr()
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: u32) {
        self.width = value;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }
    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: u32) {
        self.height = value;
    }
}
