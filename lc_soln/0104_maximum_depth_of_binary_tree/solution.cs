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
    public int MaxDepth(TreeNode root)
    {
        if (root is null)
        {
            return 0;
        }
        int result = 0;
        var q = new LinkedList<TreeNode>();
        q.AddLast(root);
        while (q.Count > 0)
        {
            int n = q.Count;
            while (n > 0)
            {
                TreeNode node = q.First.Value;
                q.RemoveFirst();
                if (node.left is not null)
                {
                    q.AddLast(node.left);
                }
                if (node.right is not null)
                {
                    q.AddLast(node.right);
                }
                n--;
            }
            result++;
        }
        return result;
    }
}
