from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        if n < 3:
            return 0
        l = 0
        r = n - 1
        lh = height[l]
        rh = height[r]
        result = 0
        while l < r:
            if lh < rh:
                l += 1
                lh = max(lh, height[l])
                result += lh - height[l]
            else:
                r -= 1
                rh = max(rh, height[r])
                result += rh - height[r]
        return result


def main(): ...


if __name__ == "__main__":
    main()
