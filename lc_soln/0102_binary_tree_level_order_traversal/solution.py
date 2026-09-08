# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        result = []
        if root is None:
            return result
        q = deque([root])
        while q:
            row = []
            n = len(q)
            for _ in range(n):
                node = q.popleft()
                row.append(node.val)
                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)
            result.append(row)
        return result


def main(): ...


if __name__ == "__main__":
    main()
