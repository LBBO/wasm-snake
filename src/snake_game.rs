use js_sys::Math;
use std::collections::VecDeque;
use std::convert::TryInto;
use tuple_conv::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[wasm_bindgen]
pub struct SnakeGame {
    width: u32,
    height: u32,
    snake_positions: VecDeque<(u32, u32)>,
    direction: Direction,
    cherry_position: (u32, u32),
}

#[wasm_bindgen]
pub fn fill_square(ctx: &CanvasRenderingContext2d, game: &SnakeGame, x: u32, y: u32, color: &str) {
    let border_width = 1;
    let inner_box_size = 7;
    let box_size = border_width + inner_box_size;

    ctx.set_fill_style(&JsValue::from(color));
    ctx.fill_rect(
        (x * box_size + border_width).into(),
        (y * box_size + border_width).into(),
        (inner_box_size).into(),
        (inner_box_size).into(),
    )
}

fn generate_random_integer(min: u32, max: u32) -> u32 {
    let random = Math::random();
    Math::floor(random * (max as f64)) as u32
}
fn generate_random_position(width: u32, height: u32) -> (u32, u32) {
    (
        generate_random_integer(0, width),
        generate_random_integer(0, height),
    )
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> SnakeGame {
        let mut queue = VecDeque::new();
        queue.push_back((50, 50));

        SnakeGame {
            width,
            height,
            snake_positions: queue,
            direction: Direction::Right,
            cherry_position: generate_random_position(width, height),
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, ctx: &CanvasRenderingContext2d) {
        let dx: i64 = match self.direction {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };
        let dy: i64 = match self.direction {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        };

        let message = format!("Snake should not be empty! {}", self.snake_positions.len());
        let &(x, y) = self.snake_positions.front().expect(&message);
        self.snake_positions.push_front((
            ((x as i64) + dx).try_into().unwrap(),
            ((y as i64) + dy).try_into().unwrap(),
        ));
        self.draw_head(ctx);

        let got_cherry = x == self.cherry_position.0 && y == self.cherry_position.1;
        if !got_cherry {
            let (x, y) = self.snake_positions.pop_back().expect("No back found");
            fill_square(ctx, self, x, y, "#000")
        } else {
            self.cherry_position = generate_random_position(self.width, self.height);
            fill_square(
                ctx,
                self,
                self.cherry_position.0,
                self.cherry_position.1,
                "#ad1457",
            )
        }
    }

    #[wasm_bindgen(getter)]
    pub fn cherry_positions(&self) -> *const u32 {
        self.cherry_position.to_vec().as_ptr()
    }

    #[wasm_bindgen]
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    #[wasm_bindgen]
    pub fn get_snake_length(&self) -> usize {
        self.snake_positions.len()
    }

    #[wasm_bindgen(getter)]
    pub fn snake_positions(&mut self) -> *const (u32, u32) {
        // let result: Vec<u32> = vec![12, 12];
        self.snake_positions.make_contiguous();
        self.snake_positions.as_slices().0.as_ptr()
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

impl SnakeGame {
    fn draw_head(&self, ctx: &CanvasRenderingContext2d) {
        let &(x, y) = self.snake_positions.front().expect("No head found");
        fill_square(ctx, self, x, y, "#bababa")
    }
}
