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
    public int KthSmallest(TreeNode root, int k)
    {
        int result = 0;
        Dfs(root, ref k, ref result);
        return result;
    }

    void Dfs(TreeNode root, ref int k, ref int result)
    {
        if (root is null)
        {
            return;
        }
        Dfs(root.left, ref k, ref result);
        if (k == 0)
        {
            return;
        }
        k--;
        if (k == 0)
        {
            result = root.val;
            return;
        }
        Dfs(root.right, ref k, ref result);
    }
}
