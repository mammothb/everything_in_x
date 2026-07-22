import sys
from pathlib import Path

CWD = Path(__file__).parent.resolve()
sys.path.append(str(CWD / "src"))

import pygame

from pydownstack.game.actions import Action
from pydownstack.game.config import GuidelineConfig
from pydownstack.game.engine import GameEngine
from pydownstack.game.events import EventBus
from pydownstack.visual.input_handler import InputHandler
from pydownstack.visual.renderer import PygameRenderer
from pydownstack.visual.yaml_settings import YamlSettings


def main():
    pygame.init()

    config = GuidelineConfig.load(
        path=CWD / "src" / "pydownstack" / "resources" / "guideline.yml"
    )
    yaml_settings = YamlSettings(CWD / "settings.yml")
    settings = yaml_settings.load()

    input_handler = InputHandler(settings)
    renderer = PygameRenderer(config)
    bus = EventBus()
    engine = GameEngine(config=config, difficulty=settings.difficulty)

    clock = pygame.time.Clock()

    while True:
        clock.tick(60)

        input_handler.push_events(pygame.event.get())
        for action in input_handler.poll_actions():
            if action == Action.QUIT:
                pygame.quit()
                sys.exit()

            for event in engine.apply_action(action):
                bus.emit(event)

        renderer.draw_frame(state=engine.get_state())


if __name__ == "__main__":
    main()
