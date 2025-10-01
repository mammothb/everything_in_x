class Solution:
    def isValid(self, s: str) -> bool:
        matching = {")": "(", "]": "[", "}": "{"}
        left = {"(", "[", "{"}
        stack = []
        for c in s:
            if c in left:
                stack.append(c)
                continue
            if not stack or stack.pop() != matching[c]:
                return False
        return not stack
