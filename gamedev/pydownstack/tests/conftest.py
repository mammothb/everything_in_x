from pathlib import Path

import pytest

from pydownstack.game.config import GuidelineConfig

_GUIDELINE = Path("src/pydownstack/resources/guideline.yml")


@pytest.fixture(scope="session")
def guideline_config() -> GuidelineConfig:
    return GuidelineConfig.load(_GUIDELINE)
