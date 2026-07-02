import argparse
import re
import textwrap
from pathlib import Path

SOLUTION_DIR = Path(__file__).parents[1].resolve()

DIFFICULTIES = ["Easy", "Medium", "Hard"]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Scaffold a new LeetCode problem folder"
    )
    parser.add_argument("number", type=int, help="LeetCode problem number")
    parser.add_argument("title", type=str, help="Problem title")
    parser.add_argument(
        "-d", "--difficulty", type=str, choices=DIFFICULTIES, required=True
    )
    parser.add_argument(
        "-t",
        "--topics",
        type=str,
        required=True,
        help="Comma-separated topics (e.g. 'Array,Hash Table')",
    )
    return parser


def main(opt: argparse.Namespace):
    topics = [s for raw in opt.topics.split(",") if (s := raw.strip())]

    slug = _title_to_slug(opt.title)
    if not slug:
        print(f"error: title '{opt.title}' produces an empty folder name")
        raise SystemExit(1)

    folder_name = f"{opt.number:04d}_{slug}"
    problem_dir = SOLUTION_DIR / folder_name

    if problem_dir.exists():
        print(f"error: folder already exists: {problem_dir}")
        raise SystemExit(1)

    problem_dir.mkdir()
    _write_problem_toml(problem_dir, opt.number, opt.title, opt.difficulty, topics)
    _write_python_stub(problem_dir)
    _write_cpp_stub(problem_dir)
    _write_rust_stub(problem_dir)

    print(f"Created: {problem_dir}")


def _title_to_slug(title: str) -> str:
    s = title.lower()
    s = s.replace("-", " ")
    s = re.sub(r"[^a-z0-9 ]", "", s)
    s = re.sub(r"\s+", "_", s.strip())
    return s


def _write_problem_toml(
    problem_dir: Path,
    number: int,
    title: str,
    difficulty: str,
    topics: list[str],
):
    topics_str = ", ".join(f'"{t}"' for t in topics)
    content = textwrap.dedent(f"""\
        number = {number}
        title = "{title}"
        difficulty = "{difficulty}"
        topics = [{topics_str}]
    """)
    (problem_dir / "problem.toml").write_text(content)


def _write_python_stub(problem_dir: Path):
    content = textwrap.dedent("""\
        class Solution:
            pass


        def main():
            ...


        if __name__ == "__main__":
            main()
    """)
    (problem_dir / "solution.py").write_text(content)


def _write_cpp_stub(problem_dir: Path):
    content = textwrap.dedent("""\
        class Solution {
        public:
        };

        int main() {
            return 0;
        }
    """)
    (problem_dir / "solution.cpp").write_text(content)


def _write_rust_stub(problem_dir: Path):
    content = textwrap.dedent("""\
        struct Solution;

        impl Solution {
            // pub fn ...
        }

        fn main() {
        }
    """)
    (problem_dir / "solution.rs").write_text(content)


if __name__ == "__main__":
    args = build_parser().parse_args()
    main(args)
