import { SnakeGame } from '../pkg/index'

const borderWidth = 1
const innerBoxSize = 7
const boxSize = borderWidth + innerBoxSize

const getCanvasWidth = (game: SnakeGame) => game.width * boxSize + borderWidth
const getCanvasHeight = (game: SnakeGame) => game.height * boxSize + borderWidth

export const initializeCanvas = (
  canvas: HTMLCanvasElement,
  game: SnakeGame,
) => {
  canvas.width = game.get_canvas_width()
  canvas.height = game.get_canvas_height()
  canvas.height = game.get_canvas_height()

  requestAnimationFrame(() => game.draw())
}
