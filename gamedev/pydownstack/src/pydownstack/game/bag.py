import random
from typing import Self, final

from pydownstack.domain.mino import Mino
from pydownstack.game.config import GuidelineConfig


@final
class Bag:
    def __init__(self, config: GuidelineConfig, seed: int | None = None):
        self.bag: list[Mino] = [mino for mino in config.pieces]
        self.size = len(self.bag)
        self.index = 0
        self.rng = random.Random(seed)
        self._refill()

    def __iter__(self) -> Self:
        self.index = 0
        return self

    def __next__(self) -> Mino:
        mino = self.bag[self.index]
        self.index = (self.index + 1) % self.size
        if self.index == 0:
            self._refill()
        return mino

    def _refill(self) -> None:
        self.rng.shuffle(self.bag)
