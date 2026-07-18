from collections import deque
from typing import List


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        q = deque()
        result = []
        start = 0
        for i, num in enumerate(nums):
            while q and nums[q[-1]] < num:
                q.pop()
            q.append(i)

            if start > q[0]:
                q.popleft()

            if i - start + 1 >= k:
                result.append(nums[q[0]])
                start += 1

        return result


def main(): ...


if __name__ == "__main__":
    main()
