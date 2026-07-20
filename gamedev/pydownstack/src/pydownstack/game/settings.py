from dataclasses import dataclass, field

from game.actions import Action


@dataclass(frozen=True)
class Settings:
    difficulty: int = 1  # 1–4, from settings.yml
    das_frames: int = 105
    arr_frames: int = 13
    keybindings: dict[Action, str] = field(
        default_factory=lambda: {
            Action.MOVE_LEFT: "left",
            Action.MOVE_RIGHT: "right",
            Action.SOFT_DROP: "down",
            Action.HARD_DROP: "space",
            Action.ROTATE_CW: "f",
            Action.ROTATE_CCW: "d",
            Action.HOLD: "s",
            Action.RESET: "f4",
        }
    )
