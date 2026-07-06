from typing import List


class Solution:
    def encode(self, strs: List[str]) -> str:
        lengths = []
        for s in strs:
            lengths.append(str(len(s)))
        return f"{','.join(lengths)}#{''.join(strs)}"

    def decode(self, s: str) -> List[str]:
        lengths, rest = s.split("#", 1)
        if not lengths:
            return []
        result = []
        start = 0
        for length in map(int, lengths.split(",")):
            result.append(rest[start : start + length])
            start += length
        return result


def main(): ...


if __name__ == "__main__":
    main()
