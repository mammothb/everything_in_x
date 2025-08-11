from typing import List


class Solution:
    def productQueries(self, n: int, queries: List[List[int]]) -> List[int]:
        prefix = [1]
        p = 1
        while n > 0:
            if n % 2 == 1:
                prefix.append(p * prefix[-1])
            p *= 2
            n //= 2

        mod = 10**9 + 7
        result = []
        for a, b in queries:
            result.append((prefix[b + 1] // prefix[a]) % mod)
        return result


print(Solution().productQueries(n=15, queries=[[0, 1], [2, 2], [0, 3]]))
print(Solution().productQueries(n=2, queries=[[0, 0]]))
