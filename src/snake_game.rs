use js_sys::Math;
use log::*;
use std::collections::VecDeque;
use std::convert::TryInto;
use tuple_conv::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
mod collision_detection;
use collision_detection::*;

static BORDER_WIDTH: f64 = 2.0;
static INNER_BOX_SIZE: f64 = 14.0;
static BOX_SIZE: f64 = BORDER_WIDTH + INNER_BOX_SIZE;

static BACKGROUND_COLOR: &str = "#000";
static GRID_COLOR: &str = "#444";
static CHERRY_COLOR: &str = "#ad1457";
static SNAKE_COLOR: &str = "#bababa";

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
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
pub fn fill_square(ctx: &CanvasRenderingContext2d, game: &SnakeGame, x: u32, y: u32, color: &str) {
    ctx.set_fill_style(&JsValue::from(color));
    ctx.fill_rect(
        x as f64 * BOX_SIZE + BORDER_WIDTH,
        y as f64 * BOX_SIZE + BORDER_WIDTH,
        INNER_BOX_SIZE,
        INNER_BOX_SIZE,
    )
}

fn generate_random_integer(min: u32, max: u32) -> u32 {
    let random = Math::random();
    Math::floor(random * (max - min) as f64) as u32 + min
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
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(generate_random_position(width, height));

        let game = SnakeGame {
            ctx,
            width,
            height,
            snake_positions: queue,
            direction: Direction::Left,
            cherry_position: generate_random_position(width, height),
        };

        game.draw();

        game
    }

    #[wasm_bindgen]
    pub fn draw(&self) {
        self.draw_grid();
        self.draw_cherries();
        self.draw_snake();
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
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

        if test_collision(&self, dx, dy) {
            panic!("ARGH");
        }

        let message = format!("Snake should not be empty! {}", self.snake_positions.len());
        let &(x, y) = self.snake_positions.front().expect(&message);
        self.snake_positions.push_front((
            ((x as i64) + dx).try_into().unwrap(),
            ((y as i64) + dy).try_into().unwrap(),
        ));
        self.draw_head();

        let got_cherry = x == self.cherry_position.0 && y == self.cherry_position.1;
        if !got_cherry {
            self.delete_tail();
        } else {
            self.cherry_position = generate_random_position(self.width, self.height);
            self.draw_cherries();
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

    #[wasm_bindgen]
    pub fn get_canvas_width(&self) -> u32 {
        self.width * BOX_SIZE as u32 + BORDER_WIDTH as u32
    }

    #[wasm_bindgen]
    pub fn get_canvas_height(&self) -> u32 {
        self.height * BOX_SIZE as u32 + BORDER_WIDTH as u32
    }
}

impl SnakeGame {
    fn draw_head(&self) {
        let &(x, y) = self.snake_positions.front().expect("No head found");
        fill_square(&self.ctx, self, x, y, SNAKE_COLOR)
    }

    fn draw_snake(&self) {
        for &position in self.snake_positions.iter() {
            let (x, y) = position;
            fill_square(&self.ctx, self, x, y, SNAKE_COLOR);
        }
    }

    fn draw_cherries(&self) {
        debug!(
            "Drawing cherry, {} {}",
            self.cherry_position.0, self.cherry_position.1
        );
        fill_square(
            &self.ctx,
            self,
            self.cherry_position.0,
            self.cherry_position.1,
            CHERRY_COLOR,
        )
    }

    fn delete_tail(&mut self) {
        let (x, y) = self.snake_positions.pop_back().expect("No tail found");
        fill_square(&self.ctx, self, x, y, BACKGROUND_COLOR)
    }

    fn draw_grid(&self) {
        // console::log(&JsValue::from(["Hello, World!"]));
        debug!("drawing grid");
        self.ctx.set_fill_style(&JsValue::from(BACKGROUND_COLOR));
        self.ctx.fill_rect(
            0.0,
            0.0,
            self.get_canvas_width() as f64,
            self.get_canvas_height() as f64,
        );

        self.ctx.set_line_width(BORDER_WIDTH.into());
        self.ctx.set_stroke_style(&JsValue::from(GRID_COLOR));

        // Draw vertical lines
        for i in 0..self.width + 1 {
            let x = (i as f64 * BOX_SIZE as f64) + 0.5 * BORDER_WIDTH as f64;

            self.ctx.move_to(x, 0.0);
            self.ctx.line_to(x, self.get_canvas_height() as f64)
        }

        // Draw horizontal lines
        for i in 0..self.height + 1 {
            let y = (i as f64 * BOX_SIZE as f64) + 0.5 * BORDER_WIDTH as f64;

            self.ctx.move_to(0.0, y);
            self.ctx.line_to(self.get_canvas_width() as f64, y)
        }

        self.ctx.stroke();
    }
}
