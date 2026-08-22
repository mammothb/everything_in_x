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
    public int DiameterOfBinaryTree(TreeNode root)
    {
        int result = 0;
        Dfs(root, ref result);
        return result;
    }

    int Dfs(TreeNode root, ref int result)
    {
        if (root is null)
        {
            return 0;
        }
        int left = Dfs(root.left, ref result);
        int right = Dfs(root.right, ref result);
        result = int.Max(result, left + right);
        return 1 + int.Max(left, right);
    }
}
