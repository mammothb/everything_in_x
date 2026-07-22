from typing import List


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        pos_and_spd = sorted(zip(position, speed), key=lambda x: -x[0])
        stack = []
        for pos, spd in pos_and_spd:
            time = (target - pos) / spd
            if stack and time <= stack[-1]:
                continue
            stack.append(time)
        return len(stack)


def main(): ...


if __name__ == "__main__":
    main()
