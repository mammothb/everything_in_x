from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        buy = float("inf")
        result = 0
        for p in prices:
            if p < buy:
                buy = p
            elif p - buy > result:
                result = max(result, p - buy)
        return result
