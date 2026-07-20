import pygame

from pydownstack.game.events import EventBus
from pydownstack.inbound_ports import GameEnginePort
from pydownstack.outbound_ports import InputPort, RendererPort
from pydownstack.visual.renderer import DEFAULT_SIZE


def main():
    pygame.init()
    screen = pygame.display.set_mode(DEFAULT_SIZE)
    clock = pygame.time.Clock()

    # input_port: InputPort = PygameInput()
    # renderer: RendererPort = PygameRenderer(screen)
    bus = EventBus()

    # engine: GameEnginePort = GameEngine(bus, seed=None)
    # scoring = ScoringSystem(bus)

    while True:
        # poll for events
        # pygame.QUIT event means the user clicked X to close your window
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        # fill the screen with a color to wipe away anything from last frame
        screen.fill("purple")

        # RENDER YOUR GAME HERE

        # flip() the display to put your work on screen
        pygame.display.flip()

        clock.tick(60)  # limits FPS to 60

    pygame.quit()


if __name__ == "__main__":
    main()
