import sys
from pathlib import Path
from typing import Literal, TypedDict

import tomllib

SOLUTION_DIR = Path(__file__).parents[1].resolve()

Difficulty = Literal["Easy", "Medium", "Hard"]
Language = Literal["cpp", "cs", "py", "rs"]
LANGUAGE_TO_FILE: dict[Language, str] = {
    "cpp": "solution.cpp",
    "cs": "solution.cs",
    "py": "solution.py",
    "rs": "solution.rs",
}


class Problem(TypedDict):
    number: int
    title: str
    difficulty: Difficulty
    topics: list[str]
    languages: list[Language]


def main():
    problems: list[Problem] = []
    for path in SOLUTION_DIR.iterdir():
        if not path.is_dir():
            continue
        if _try_parse_problem_number(path.name) is None:
            continue

        problem_toml = path / "problem.toml"
        if not problem_toml.exists():
            print(f"warning: {problem_toml} not found", file=sys.stdout)
            continue

        problems.append(_load_problem(problem_toml))

    _render_readme(problems)


def _try_parse_problem_number(name: str) -> int | None:
    try:
        parts = name.split("_")
        return int(parts[0])
    except:
        return None


def _load_problem(path: Path) -> Problem:
    with path.open(mode="rb") as f:
        raw = tomllib.load(f)

    return Problem(
        number=int(raw["number"]),
        title=raw["title"],
        difficulty=raw["difficulty"],
        topics=raw["topics"],
        languages=[
            lang
            for lang, file in LANGUAGE_TO_FILE.items()
            if (path.parent / file).exists()
        ],
    )


def _render_readme(problems: list[Problem]) -> None:
    with (SOLUTION_DIR / "README.md").open("w") as f:
        f.write("# lc_soln\n")
        f.write("| Name | Difficulty | Topics | Language |\n")
        f.write("| ---- | ---------- | ------ | -------- |\n")
        for problem in sorted(problems, key=lambda p: p["number"]):
            f.write(
                f"| {problem['number']}. {problem['title']} | {problem['difficulty']} | {', '.join(problem['topics'])} | {', '.join(problem['languages'])} |\n"
            )


if __name__ == "__main__":
    main()
