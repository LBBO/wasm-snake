import { SnakeGame } from '../pkg/index'
import { setupEventHandlers } from './eventhandlers'
import { GameLoop } from './gameloop'
import { initializeCanvas } from './renderer'

const canvas = document.querySelector<HTMLCanvasElement>('.game-field')
const ctx = canvas?.getContext('2d')

if (canvas && ctx) {
  const game = new SnakeGame(ctx, 50, 50)
  const gameLoop = new GameLoop(game)
  initializeCanvas(canvas, game)
  setupEventHandlers(game, gameLoop)

  gameLoop.play()
} else {
  console.error('Canvas not found!')
}
