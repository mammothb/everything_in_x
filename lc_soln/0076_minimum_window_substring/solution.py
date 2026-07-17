class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if len(t) > len(s):
            return ""

        need_map = {}
        for c in t:
            need_map[c] = need_map.get(c, 0) + 1

        need = len(need_map)  # unique chars to satisfy
        have = 0
        min_len = float("inf")
        result = ""
        left = 0

        for right, c in enumerate(s):
            if c in need_map:
                need_map[c] -= 1
                if need_map[c] == 0:
                    have += 1

            # Shrink window from left while we have excess
            while left <= right and need_map.get(s[left], -1) < 0:
                if s[left] in need_map:
                    need_map[s[left]] += 1
                left += 1

            if have == need and right - left + 1 < min_len:
                min_len = right - left + 1
                result = s[left : right + 1]

        return result


def main(): ...


if __name__ == "__main__":
    main()
