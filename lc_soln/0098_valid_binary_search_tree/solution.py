# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return self.dfs(root, float("-inf"), float("inf"))

    def dfs(self, root: Optional[TreeNode], min_val: int, max_val: int):
        if root is None:
            return True
        if not (min_val < root.val < max_val):
            return False

        return self.dfs(root.left, min_val, root.val) and self.dfs(
            root.right, root.val, max_val
        )


def main(): ...


if __name__ == "__main__":
    main()
