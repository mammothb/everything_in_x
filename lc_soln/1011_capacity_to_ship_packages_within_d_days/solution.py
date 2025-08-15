from typing import List


class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        def time_taken(weights, capacity):
            result = 1
            curr = 0
            for w in weights:
                curr += w
                if curr > capacity:
                    curr = w
                    result += 1
            return result

        l = max(weights)
        r = sum(weights)

        result = -1
        while l <= r:
            mid = l + (r - l) // 2
            if time_taken(weights, mid) <= days:
                result = mid
                r = mid - 1
            else:
                l = mid + 1
        return result
