import { SnakeGame } from '../pkg/index'
import { draw, initializeCanvas } from './renderer'

const canvas = document.querySelector<HTMLCanvasElement>('.game-field')
const ctx = canvas?.getContext('2d')

if (canvas && ctx) {
  const game = new SnakeGame(150, 100)
  initializeCanvas(canvas, game)
  draw(ctx, game)
} else {
  console.error('Canvas not found!')
}
