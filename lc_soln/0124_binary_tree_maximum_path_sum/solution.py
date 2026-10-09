# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        result = [float("-inf")]
        self.dfs(root, result)
        return result[0]

    def dfs(self, root, result):
        if root is None:
            return 0

        left_tree = self.dfs(root.left, result)
        right_tree = self.dfs(root.right, result)

        max_single = max(root.val, root.val + max(left_tree, right_tree))
        max_sum = max(root.val + left_tree + right_tree, max_single)
        result[0] = max(result[0], max_sum)
        return max_single


def main(): ...


if __name__ == "__main__":
    main()
