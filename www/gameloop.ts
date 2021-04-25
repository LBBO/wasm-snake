import { GameStatus, SnakeGame } from '../pkg/index'

export class GameLoop {
  private counter = 1
  static readonly maxCounter = 7
  animationFrameId: ReturnType<typeof requestAnimationFrame> | null = null

  constructor(private game: SnakeGame) {}

  toggle = () => {
    switch (this.game.status as GameStatus) {
      case GameStatus.Running:
        this.pause()
        break
      case GameStatus.Paused:
        this.play()
        break
      default:
        this.game.reset()
        this.play()
        break
    }
  }

  play = (): GameStatus => {
    this.game.status = GameStatus.Running
    this.counter++
    this.counter = this.counter % GameLoop.maxCounter

    if (this.counter === 0) {
      const newStatus = this.game.tick()

      if (newStatus === GameStatus.Lost) {
        alert('Lost!!')
        this.counter = 0
        this.animationFrameId = null

        const button = document.querySelector<HTMLButtonElement>('.play-pause')
        if (button) {
          button.textContent = 'Restart'
        }
      }
    }

    if (this.game.status === GameStatus.Running) {
      this.animationFrameId = requestAnimationFrame(this.play)
    } else {
      console.log(this.game.status)
    }

    return this.game.status
  }

  pause = () => {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId)
    }

    this.game.status = GameStatus.Paused
    this.animationFrameId = null
  }

  get isPlaying(): boolean {
    return this.game.status === GameStatus.Running
  }
}
