class Solution:
    def getLengthOfOptimalCompression(self, s: str, k: int) -> int:
        def get_len(count):
            if count == 0:
                return 0
            if count == 1:
                return 1
            if count < 10:
                return 2
            if count < 100:
                return 3
            return 4

        n = len(s)
        dp = [[float("inf")] * (k + 1) for _ in range(n + 1)]
        for j in range(k + 1):
            dp[0][j] = 0

        for i in range(1, n + 1):
            for j in range(k + 1):
                if j > 0:
                    dp[i][j] = dp[i - 1][j - 1]
                removed = 0
                count = 0
                for p in range(i, 0, -1):
                    if s[p - 1] == s[i - 1]:
                        count += 1
                    else:
                        removed += 1
                        if removed > j:
                            break
                    dp[i][j] = min(dp[i][j], dp[p - 1][j - removed] + get_len(count))
        return dp[-1][-1]
