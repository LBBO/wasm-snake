import { SnakeGame } from '../pkg/index'
import { setupEventHandlers } from './eventhandlers'
import { draw, initializeCanvas } from './renderer'

const canvas = document.querySelector<HTMLCanvasElement>('.game-field')
const ctx = canvas?.getContext('2d')

if (canvas && ctx) {
  const game = new SnakeGame(150, 100)
  initializeCanvas(canvas, game)
  setupEventHandlers(game)
  draw(ctx, game)

  let counter = 1
  const maxCounter = 10
  const gameLoop = () => {
    counter++
    counter = counter % maxCounter

    if (counter === 0) {
      game.tick()
      draw(ctx, game)
    }

    requestAnimationFrame(gameLoop)
  }
  gameLoop()
} else {
  console.error('Canvas not found!')
}
