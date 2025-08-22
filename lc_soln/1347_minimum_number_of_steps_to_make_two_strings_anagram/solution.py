class Solution:
    def minSteps(self, s: str, t: str) -> int:
        if s == t:
            return 0

        counter_s = Counter(s)
        counter_t = Counter(t)
        result = 0
        for c, v in counter_s.items():
            if c not in counter_t:
                result += v
            elif (diff := v - counter_t[c]) > 0:
                result += diff
        return result
