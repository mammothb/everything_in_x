from enum import Enum, auto


class Action(Enum):
    """Game actions."""

    MOVE_LEFT = auto()
    MOVE_RIGHT = auto()
    SOFT_DROP = auto()
    HARD_DROP = auto()
    ROTATE_CW = auto()
    ROTATE_CCW = auto()
    HOLD = auto()
    RESET = auto()
    PAUSE = auto()
    QUIT = auto()
