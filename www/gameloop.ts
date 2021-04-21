import { SnakeGame } from '../pkg/index'
import { draw } from './renderer'

export class GameLoop {
  private counter = 1
  static readonly maxCounter = 4
  animationFrameId: ReturnType<typeof requestAnimationFrame> | null = null

  constructor(private ctx: CanvasRenderingContext2D, private game: SnakeGame) {
    draw(this.ctx, this.game)
  }

  toggle = () => {
    if (this.isPlaying) {
      this.pause()
    } else {
      this.play()
    }
  }

  play = () => {
    this.counter++
    this.counter = this.counter % GameLoop.maxCounter

    if (this.counter === 0) {
      this.game.tick(this.ctx)
      // draw(this.ctx, this.game)
    }

    this.animationFrameId = requestAnimationFrame(this.play)
  }

  pause = () => {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId)
    }

    this.animationFrameId = null
  }

  get isPlaying(): boolean {
    return this.animationFrameId !== null
  }
}
