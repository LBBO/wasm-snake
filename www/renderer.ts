import { fill_square, SnakeGame } from '../pkg/index'
import { memory } from '../pkg/index_bg.wasm'

const borderWidth = 1
const innerBoxSize = 7
const boxSize = borderWidth + innerBoxSize

const backgroundColor = '#000'
const gridColor = '#444'
const cherryColor = '#ad1457'
const snakeColor = '#bababa'

const getCanvasWidth = (game: SnakeGame) => game.width * boxSize + borderWidth
const getCanvasHeight = (game: SnakeGame) => game.height * boxSize + borderWidth

const drawGrid = (ctx: CanvasRenderingContext2D, game: SnakeGame) => {
  ctx.lineWidth = borderWidth
  ctx.strokeStyle = gridColor

  for (let i = 0; i <= game.width; i++) {
    const x = i * boxSize + 0.5 * borderWidth

    ctx.moveTo(x, 0)
    ctx.lineTo(x, getCanvasHeight(game))
  }

  for (let i = 0; i <= game.height; i++) {
    const y = i * boxSize + 0.5 * borderWidth

    ctx.moveTo(0, y)
    ctx.lineTo(getCanvasWidth(game), y)
  }

  ctx.stroke()
}

const drawCherries = (ctx: CanvasRenderingContext2D, game: SnakeGame) => {
  const cherriesPtr = game.cherry_positions
  const [cherryX, cherryY] = new Uint32Array(memory.buffer, cherriesPtr, 2)

  ctx.fillStyle = cherryColor
  fill_square(ctx, game, cherryX, cherryY, cherryColor)
}

const drawSnake = (ctx: CanvasRenderingContext2D, game: SnakeGame) => {
  const snakePositionsPtr = game.snake_positions
  const snakeLength = game.get_snake_length()
  const snakePositions = new Uint32Array(
    memory.buffer,
    snakePositionsPtr,
    snakeLength * 2,
  )

  ctx.fillStyle = snakeColor
  for (let i = 0; i < snakeLength; i++) {
    const x = snakePositions[i * 2]
    const y = snakePositions[i * 2 + 1]

    ctx.fillRect(
      x * boxSize + borderWidth,
      y * boxSize + borderWidth,
      innerBoxSize,
      innerBoxSize,
    )
  }
}

export const draw = (ctx: CanvasRenderingContext2D, game: SnakeGame) => {
  // Reset canvas
  ctx.fillStyle = backgroundColor
  ctx.fillRect(0, 0, getCanvasWidth(game), getCanvasHeight(game))

  drawGrid(ctx, game)
  drawCherries(ctx, game)
  drawSnake(ctx, game)
}

export const initializeCanvas = (
  canvas: HTMLCanvasElement,
  game: SnakeGame,
) => {
  canvas.width = getCanvasWidth(game)
  canvas.height = getCanvasHeight(game)
}
