# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        return self.dfs(root)[0]

    def dfs(self, root):
        if root is None:
            return True, 0
        left_balanced, left_height = self.dfs(root.left)
        right_balanced, right_height = self.dfs(root.right)
        return left_balanced and right_balanced and abs(
            left_height - right_height
        ) < 2, 1 + max(left_height, right_height)


def main(): ...


if __name__ == "__main__":
    main()
