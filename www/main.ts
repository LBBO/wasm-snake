import { SnakeGame } from '../pkg/index'
import { setupEventHandlers } from './eventhandlers'
import { GameLoop } from './gameloop'
import { draw, initializeCanvas } from './renderer'

const canvas = document.querySelector<HTMLCanvasElement>('.game-field')
const ctx = canvas?.getContext('2d')

if (canvas && ctx) {
  const game = new SnakeGame(150, 100)
  const gameLoop = new GameLoop(ctx, game)
  initializeCanvas(canvas, game)
  setupEventHandlers(game, gameLoop)
  draw(ctx, game)

  gameLoop.play()
} else {
  console.error('Canvas not found!')
}
