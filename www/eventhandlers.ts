import { Direction, SnakeGame } from '../pkg/index'
import { GameLoop } from './gameloop'

const setupPlayPauseButton = (gameLoop: GameLoop) => {
  const button = document.querySelector<HTMLButtonElement>('.play-pause')
  if (button) {
    const updateButtonText = () => {
      button.textContent = gameLoop.isPlaying ? '⏸' : '⏯'
    }

    button.addEventListener('click', () => {
      gameLoop.toggle()
      updateButtonText()
    })

    updateButtonText()
  }
}

export const setupEventHandlers = (game: SnakeGame, gameLoop: GameLoop) => {
  setupPlayPauseButton(gameLoop)

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
