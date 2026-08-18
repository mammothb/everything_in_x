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
    public TreeNode InvertTree(TreeNode root)
    {
        if (root is null)
        {
            return root;
        }
        var stack = new Stack<TreeNode>();
        stack.Push(root);
        while (stack.Count > 0)
        {
            TreeNode node = stack.Pop();
            (node.left, node.right) = (node.right, node.left);
            if (node.left is not null)
            {
                stack.Push(node.left);
            }
            if (node.right is not null)
            {
                stack.Push(node.right);
            }
        }
        return root;
    }
}
