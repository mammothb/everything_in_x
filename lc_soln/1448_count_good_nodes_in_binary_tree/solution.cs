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
    public int GoodNodes(TreeNode root)
    {
        return Dfs(root, int.MinValue);
    }

    private int Dfs(TreeNode node, int maxVal)
    {
        if (node is null)
        {
            return 0;
        }
        maxVal = Math.Max(maxVal, node.val);
        int result = node.val >= maxVal ? 1 : 0;
        return result + Dfs(node.left, maxVal) + Dfs(node.right, maxVal);
    }
}
