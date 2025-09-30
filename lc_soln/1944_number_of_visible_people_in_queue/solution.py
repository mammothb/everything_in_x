from typing import List


class Solution:
    def canSeePersonsCount(self, heights: List[int]) -> List[int]:
        n = len(heights)
        result = [0] * n
        stack = []
        for i in range(n - 1, -1, -1):
            while stack and stack[-1] <= heights[i]:
                result[i] += 1
                stack.pop()
            if stack:
                result[i] += 1
            stack.append(heights[i])
        return result
