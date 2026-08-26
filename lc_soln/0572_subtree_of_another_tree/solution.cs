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
    public bool IsSubtree(TreeNode root, TreeNode subRoot)
    {
        if (root is null && subRoot is null)
        {
            return true;
        }
        if (root is null || subRoot is null)
        {
            return false;
        }
        if (SameTree(root, subRoot))
        {
            return true;
        }
        return IsSubtree(root.left, subRoot) || IsSubtree(root.right, subRoot);
    }

    bool SameTree(TreeNode root1, TreeNode root2)
    {
        if (root1 is null && root2 is null)
        {
            return true;
        }
        if (root1 is null || root2 is null)
        {
            return false;
        }
        return root1.val == root2.val
            && SameTree(root1.left, root2.left)
            && SameTree(root1.right, root2.right);
    }
}
