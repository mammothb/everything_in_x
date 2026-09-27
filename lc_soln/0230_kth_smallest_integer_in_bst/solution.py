# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        result = [k, None]
        self.dfs(root, result)
        return result[1]

    def dfs(self, root, result):
        if root is None:
            return

        self.dfs(root.left, result)
        if result[0] == 0:
            return

        result[0] -= 1
        if result[0] == 0:
            result[1] = root.val
            return

        self.dfs(root.right, result)


def main(): ...


if __name__ == "__main__":
    main()
