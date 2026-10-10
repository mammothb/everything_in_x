# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Codec:
    # Encodes a tree to a single string.
    def serialize(self, root: Optional[TreeNode]) -> str:
        result = [""]
        stack = [root]
        while stack:
            node = stack.pop()
            if node is None:
                result.append("N")
            else:
                result.append(str(node.val))
                stack.append(node.right)
                stack.append(node.left)
        return ",".join(result)

    # Decodes your encoded data to tree.
    def deserialize(self, data: str) -> Optional[TreeNode]:
        i = 0

        def dfs(values):
            nonlocal i
            i += 1
            if values[i] == "N":
                return None
            root = TreeNode(int(values[i]))
            root.left = dfs(values)
            root.right = dfs(values)
            return root

        values = data.split(",")
        return dfs(values)


def main(): ...


if __name__ == "__main__":
    main()
