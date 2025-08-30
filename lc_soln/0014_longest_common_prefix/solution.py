from typing import List


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        result = ""
        i = 0
        while True:
            if i >= len(strs[0]):
                return result
            c = strs[0][i]
            for s in strs:
                if i >= len(s):
                    return result
                if s[i] != c:
                    return result
            result += c
            i += 1
