# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        result = [0]
        self.dfs(root, result)
        return result[0]

    def dfs(self, root: Optional[TreeNode], result: list[int]) -> int:
        if root is None:
            return 0
        left = self.dfs(root.left, result)
        right = self.dfs(root.right, result)
        result[0] = max(result[0], left + right)
        return 1 + max(left, right)


def main(): ...


if __name__ == "__main__":
    main()
