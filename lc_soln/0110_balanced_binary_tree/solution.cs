/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     public int val;
 *     public TreeNode left;
 *     public TreeNode right;
 *     public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

public class Solution
{
    public bool IsBalanced(TreeNode root)
    {
        return Dfs(root).isBalanced;
    }

    (bool isBalanced, int height) Dfs(TreeNode root)
    {
        if (root is null)
        {
            return (true, 0);
        }
        (bool leftBalanced, int leftHeight) = Dfs(root.left);
        (bool rightBalanced, int rightHeight) = Dfs(root.right);
        return (
            leftBalanced && rightBalanced && Math.Abs(leftHeight - rightHeight) < 2,
            1 + Math.Max(leftHeight, rightHeight)
        );
    }
}
