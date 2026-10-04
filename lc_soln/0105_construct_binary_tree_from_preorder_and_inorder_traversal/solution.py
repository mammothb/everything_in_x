# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        pre_idx = [0]
        in_to_idx = {v: i for i, v in enumerate(inorder)}
        root = self.build(preorder, inorder, 0, len(inorder) - 1, pre_idx, in_to_idx)
        return root

    def build(self, preorder, inorder, in_start, in_stop, pre_idx, in_to_idx):
        if in_start > in_stop:
            return None
        root = TreeNode(preorder[pre_idx[0]])
        pre_idx[0] += 1
        root.left = self.build(
            preorder, inorder, in_start, in_to_idx[root.val] - 1, pre_idx, in_to_idx
        )
        root.right = self.build(
            preorder, inorder, in_to_idx[root.val] + 1, in_stop, pre_idx, in_to_idx
        )
        return root


def main(): ...


if __name__ == "__main__":
    main()
