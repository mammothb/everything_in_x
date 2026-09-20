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
    public bool IsValidBST(TreeNode root)
    {
        return Dfs(root, int.MinValue, int.MaxValue);
    }

    bool Dfs(TreeNode root, int minVal, int maxVal)
    {
        if (root is null)
        {
            return true;
        }
        if (!(minVal < root.val && root.val < maxVal))
        {
            return false;
        }
        return Dfs(root.left, minVal, root.val) && Dfs(root.right, root.val, maxVal);
    }
}
