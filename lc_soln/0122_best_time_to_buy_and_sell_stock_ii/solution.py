from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        result = 0
        for i in range(len(prices) - 1):
            if (profit := prices[i + 1] - prices[i]) > 0:
                result += profit
        return result
