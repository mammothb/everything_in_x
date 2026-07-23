from typing import List


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        result = 0
        n = len(heights)
        lefts = [-1] * n
        stack = []
        for i, h in enumerate(heights):
            while stack and heights[stack[-1]] >= h:
                stack.pop()
            if stack:
                lefts[i] = stack[-1]
            stack.append(i)

        stack = []
        rights = [n] * n
        for i, h in reversed(list(enumerate(heights))):
            while stack and heights[stack[-1]] >= h:
                stack.pop()
            if stack:
                rights[i] = stack[-1]
            stack.append(i)

        result = 0
        for i in range(n):
            result = max(result, heights[i] * (rights[i] - lefts[i] - 1))
        return result


def main(): ...


if __name__ == "__main__":
    main()
