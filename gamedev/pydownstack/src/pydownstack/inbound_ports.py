from abc import ABC, abstractmethod

from pydownstack.game.actions import Action
from pydownstack.game.events import GameEvent


class GameEnginePort(ABC):
    @abstractmethod
    def apply_action(self, action: Action) -> list[GameEvent]: ...

    @abstractmethod
    def tick(self) -> list[GameEvent]: ...

    # @abstractmethod
    # def get_state(self) -> GameState: ...

    @abstractmethod
    def reset(self) -> None: ...
