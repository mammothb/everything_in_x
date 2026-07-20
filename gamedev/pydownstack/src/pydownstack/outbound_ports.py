from abc import ABC, abstractmethod

from pydownstack.game.actions import Action
from pydownstack.game.game_state import GameState
from pydownstack.game.settings import Settings


class RendererPort(ABC):
    @abstractmethod
    def draw_frame(self, state: GameState) -> None: ...


class InputPort(ABC):
    @abstractmethod
    def poll_actions(self) -> list[Action]: ...


class SettingsPort(ABC):
    @abstractmethod
    def load(self) -> Settings: ...

    @abstractmethod
    def save(self, settings: Settings) -> None: ...
