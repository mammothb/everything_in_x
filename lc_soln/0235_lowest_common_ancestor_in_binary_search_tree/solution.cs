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
    public TreeNode LowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q)
    {
        while (root is not null)
        {
            if (p.val > root.val && q.val > root.val)
            {
                root = root.right;
            }
            else if (p.val < root.val && q.val < root.val)
            {
                root = root.left;
            }
            else
            {
                return root;
            }
        }
        return null;
    }
}
