from pathlib import Path
from typing import final, override

import yaml

from pydownstack.game.actions import Action
from pydownstack.game.settings import Settings
from pydownstack.outbound_ports import SettingsPort


@final
class YamlSettings(SettingsPort):
    def __init__(self, path: Path) -> None:
        self._path = path

    @override
    def load(self) -> Settings:
        raw = yaml.safe_load(self._path.open())
        difficulty = self._validate_difficulty(raw.get("difficulty", 1))
        das = self._validate_positive(raw.get("das_frames", 105))
        arr = self._validate_positive(raw.get("arr_frames", 13))
        if "keybindings" in raw:
            return Settings(
                difficulty=difficulty,
                das_frames=das,
                arr_frames=arr,
                keybindings=self._parse_keybindings(raw["keybindings"]),
            )
        return Settings(difficulty=difficulty, das_frames=das, arr_frames=arr)

    @override
    def save(self, settings: Settings) -> None:
        pass

    @staticmethod
    def _parse_keybindings(raw: dict[str, str]) -> dict[Action, str]:
        result: dict[Action, str] = {}
        for name, key_name in raw.items():
            try:
                result[Action[name.upper()]] = key_name
            except KeyError:
                raise ValueError(f"Unknown action: {name}")
        return result

    @staticmethod
    def _validate_difficulty(value: object) -> int:
        if not isinstance(value, int) or not 1 <= value <= 4:
            raise ValueError(f"difficulty must be 1-4, got {value!r}")
        return value

    @staticmethod
    def _validate_positive(value: object) -> int:
        if not isinstance(value, int) or value < 1:
            raise ValueError(f"must be positive int, got {value!r}")
        return value
