import { SnakeGame } from '../pkg/index'

export class GameLoop {
  private counter = 1
  static readonly maxCounter = 7
  animationFrameId: ReturnType<typeof requestAnimationFrame> | null = null

  constructor(private game: SnakeGame) {}

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
      this.game.tick()
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
