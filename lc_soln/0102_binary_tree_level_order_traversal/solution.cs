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
    public List<List<int>> LevelOrder(TreeNode root)
    {
        List<List<int>> result = [];
        LinkedList<TreeNode> q = [];
        if (root is not null)
        {
            q.AddLast(root);
        }
        while (q.Count > 0)
        {
            int n = q.Count;
            var row = new List<int>(n);
            while (n-- > 0)
            {
                TreeNode node = q.First.Value;
                q.RemoveFirst();
                row.Add(node.val);
                if (node.left is not null)
                {
                    q.AddLast(node.left);
                }
                if (node.right is not null)
                {
                    q.AddLast(node.right);
                }
            }
            result.Add(row);
        }
        return result;
    }
}
