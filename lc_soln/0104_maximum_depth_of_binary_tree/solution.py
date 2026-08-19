# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        result = 0
        q = deque([root])
        while q:
            n = len(q)
            while n > 0:
                node = q.popleft()
                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)
                n -= 1
            result += 1
        return result


def main(): ...


if __name__ == "__main__":
    main()
