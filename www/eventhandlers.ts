import { Direction, SnakeGame } from '../pkg/index'

export const setupEventHandlers = (game: SnakeGame) => {
  document.addEventListener('keydown', (e) => {
    console.log(e.key)
    switch (e.key) {
      case 'ArrowUp':
        game.set_direction(Direction.Up)
        break
      case 'ArrowDown':
        game.set_direction(Direction.Down)
        break
      case 'ArrowLeft':
        game.set_direction(Direction.Left)
        break
      case 'ArrowRight':
        game.set_direction(Direction.Right)
        break
    }
  })
}
