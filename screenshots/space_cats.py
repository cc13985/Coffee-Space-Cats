from dataclasses import dataclass
from enum import Enum
import math


class CatStatus(Enum):
    SLEEPING = "sleeping"
    CAFFEINATED = "caffeinated"
    EXPLORING = "exploring"
    LOST_IN_SPACE = "lost_in_space"


@dataclass
class SpaceCat:
    name: str
    coffees_consumed: int = 0
    coordinates: tuple[float, float, float] = (0.0, 0.0, 0.0)

    def drink_coffee(self, cups: int = 1) -> None:
        """Fuel up before launch."""
        self.coffees_consumed += cups
        print(f"{self.name} drank {cups} cup(s). Total: {self.coffees_consumed} ☕")

    @property
    def status(self) -> CatStatus:
        if self.coffees_consumed == 0:
            return CatStatus.SLEEPING
        elif self.coffees_consumed < 3:
            return CatStatus.CAFFEINATED
        return CatStatus.EXPLORING

    def distance_from_earth(self) -> float:
        x, y, z = self.coordinates
        return math.sqrt(x**2 + y**2 + z**2)

    def warp(self, dx: float, dy: float, dz: float) -> None:
        if self.status == CatStatus.SLEEPING:
            raise RuntimeError("Cat needs coffee before warping!")
        x, y, z = self.coordinates
        self.coordinates = (x + dx, y + dy, z + dz)
        print(f"📡 Warped to {self.coordinates}")


# Launch sequence
cat = SpaceCat(name="Nebula", coordinates=(1.2, 3.4, -0.8))
cat.drink_coffee(cups=2)
cat.warp(dx=10.5, dy=-3.2, dz=7.1)
print(f"Distance from Earth: {cat.distance_from_earth():.2f} light-years")
print(f"Status: {cat.status.value}")
