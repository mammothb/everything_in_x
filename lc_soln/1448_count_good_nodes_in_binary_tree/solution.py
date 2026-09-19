# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        stack = [(root, root.val)]
        result = 0
        while stack:
            node, max_val = stack.pop()
            max_val = max(max_val, node.val)
            if node.val >= max_val:
                result += 1
            if node.left is not None:
                stack.append((node.left, max_val))
            if node.right is not None:
                stack.append((node.right, max_val))
        return result


def main(): ...


if __name__ == "__main__":
    main()
