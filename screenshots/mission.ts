interface Planet {
  name: string
  distanceLy: number
  hasCoffee: boolean
}

type MissionStatus = "pending" | "active" | "complete" | "aborted"

const KNOWN_PLANETS: Planet[] = [
  { name: "Espresso Prime", distanceLy: 4.2, hasCoffee: true },
  { name: "Void IX",        distanceLy: 12.7, hasCoffee: false },
  { name: "Meow Station",   distanceLy: 0.3,  hasCoffee: true },
]

class SpaceMission {
  private status: MissionStatus = "pending"
  private log: string[] = []

  constructor(
    public readonly missionName: string,
    private crew: string[],
    private destination: Planet,
  ) {}

  async launch(): Promise<void> {
    if (!this.destination.hasCoffee) {
      throw new Error(`Aborting — no coffee at ${this.destination.name}`)
    }
    this.status = "active"
    this.log.push(`🚀 Launched toward ${this.destination.name}`)
    await this.travel(this.destination.distanceLy)
    this.status = "complete"
  }

  private async travel(distance: number): Promise<void> {
    const duration = distance * 365 // days per light-year
    this.log.push(`Traveling ${distance} ly (~${duration} days)`)
    return new Promise(resolve => setTimeout(resolve, 100))
  }

  get summary(): string {
    return [
      `Mission: ${this.missionName}`,
      `Crew:    ${this.crew.join(", ")}`,
      `Status:  ${this.status}`,
      ...this.log.map(l => `  › ${l}`),
    ].join("\n")
  }
}

const mission = new SpaceMission(
  "Operation Latte",
  ["Nebula", "Cosmo", "Biscuit"],
  KNOWN_PLANETS[0],
)

mission.launch().then(() => console.log(mission.summary))
