use crate::SnakeGame;

pub fn test_collision(game: &SnakeGame, dx: i64, dy: i64) -> bool {
    let head = game.snake_positions.front().expect("Head expected");
    let new_x = head.0 as i64 + dx;
    let new_y = head.1 as i64 + dy;

    let mut touching_self = false;
    for &(x, y) in game.snake_positions.iter() {
        touching_self = touching_self || (new_x == x as i64 && new_y == y as i64);
    }

    touching_self
        || new_x < 0
        || new_y < 0
        || new_x >= game.width as i64
        || new_y >= game.height as i64
}
