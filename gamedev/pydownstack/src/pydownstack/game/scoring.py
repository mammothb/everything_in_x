from typing import final

from pydownstack.game.config import GuidelineConfig
from pydownstack.game.events import EventBus, HardDropped, LineCleared


@final
class ScoringSystem:
    def __init__(self, bus: EventBus, config: GuidelineConfig) -> None:
        self._config = config
        self.score = 0
        self.lines = 0
        bus.subscribe(LineCleared, self._on_line_cleared)
        bus.subscribe(HardDropped, self._on_hard_dropped)

    def _on_hard_dropped(self, event: HardDropped) -> None:
        self.score += event.distance * 2

    def _on_line_cleared(self, event: LineCleared) -> None:
        points = self._config.scoring.get(event.count, 0)
        self.score += points
        self.lines += event.count
