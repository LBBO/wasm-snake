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
#[derive(Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

static STARTING_DIRECTION: Direction = Direction::Left;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    Paused = 0,
    Running = 1,
    Won = 2,
    Lost = 3,
}

#[wasm_bindgen]
pub struct SnakeGame {
    width: u32,
    height: u32,
    snake_positions: VecDeque<(u32, u32)>,
    direction: Direction,
    cherry_position: (u32, u32),
    ctx: CanvasRenderingContext2d,
    status: GameStatus,
}

#[wasm_bindgen]
pub fn fill_square(ctx: &CanvasRenderingContext2d, x: u32, y: u32, color: &str) {
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
            direction: STARTING_DIRECTION,
            cherry_position: generate_random_position(width, height),
            status: GameStatus::Paused,
        };

        game.draw();

        game
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back(generate_random_position(self.width, self.height));
        self.snake_positions = queue;

        self.direction = STARTING_DIRECTION;
        self.cherry_position = generate_random_position(self.width, self.height);
        self.status = GameStatus::Paused;
        self.draw();
    }

    #[wasm_bindgen]
    pub fn draw(&self) {
        self.draw_grid();
        self.draw_cherries();
        self.draw_snake();
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> GameStatus {
        let (next_x, next_y) = self.compute_next_head_position();

        if test_collision(&self, next_x, next_y) {
            self.status = GameStatus::Lost;
        } else if self.status == GameStatus::Running {
            let message = format!("Snake should not be empty! {}", self.snake_positions.len());
            let &(x, y) = self.snake_positions.front().expect(&message);
            self.snake_positions
                .push_front((next_x as u32, next_y as u32));
            self.draw_head();

            let got_cherry = x == self.cherry_position.0 && y == self.cherry_position.1;
            if !got_cherry {
                self.delete_tail();
            } else {
                self.cherry_position = generate_random_position(self.width, self.height);

                if self.get_snake_length() == (self.width * self.height).try_into().unwrap() {
                    self.status = GameStatus::Won;
                } else {
                    self.draw_cherries();
                }
            }
        }

        self.status
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

    #[wasm_bindgen(getter=status)]
    pub fn get_status(&self) -> GameStatus {
        self.status
    }

    #[wasm_bindgen(setter=status)]
    pub fn set_status(&mut self, new_status: GameStatus) {
        self.status = new_status;
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

// Private methods
impl SnakeGame {
    fn compute_next_head_position(&self) -> (i64, i64) {
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

        ((x as i64) + dx, (y as i64) + dy)
    }

    fn draw_head(&self) {
        let &(x, y) = self.snake_positions.front().expect("No head found");
        fill_square(&self.ctx, x, y, SNAKE_COLOR)
    }

    fn draw_snake(&self) {
        for &position in self.snake_positions.iter() {
            let (x, y) = position;
            fill_square(&self.ctx, x, y, SNAKE_COLOR);
        }
    }

    fn draw_cherries(&self) {
        debug!(
            "Drawing cherry, {} {}",
            self.cherry_position.0, self.cherry_position.1
        );
        fill_square(
            &self.ctx,
            self.cherry_position.0,
            self.cherry_position.1,
            CHERRY_COLOR,
        )
    }

    fn delete_tail(&mut self) {
        let (x, y) = self.snake_positions.pop_back().expect("No tail found");
        fill_square(&self.ctx, x, y, BACKGROUND_COLOR)
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
