use crate::SnakeGame;

pub fn test_collision(game: &SnakeGame, new_x: i64, new_y: i64) -> bool {
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
