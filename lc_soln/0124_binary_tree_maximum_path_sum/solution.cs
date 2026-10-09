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
    public int MaxPathSum(TreeNode root)
    {
        int result = int.MinValue;
        Dfs(root, ref result);
        return result;
    }

    int Dfs(TreeNode root, ref int result)
    {
        if (root is null)
        {
            return 0;
        }

        int leftTree = Dfs(root.left, ref result);
        int rightTree = Dfs(root.right, ref result);

        int maxSingle = Math.Max(root.val, root.val + Math.Max(Math.Max(leftTree, rightTree), 0));
        int maxSum = Math.Max(maxSingle, root.val + leftTree + rightTree);
        result = Math.Max(result, maxSum);
        return maxSingle;
    }
}
