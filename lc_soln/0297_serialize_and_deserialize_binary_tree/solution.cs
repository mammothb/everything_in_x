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

public class Codec
{
    // Encodes a tree to a single string.
    public string Serialize(TreeNode root)
    {
        List<string> result = [""];
        var stack = new Stack<TreeNode>();
        stack.Push(root);
        while (stack.Count > 0)
        {
            TreeNode node = stack.Pop();
            if (node is null)
            {
                result.Add("N");
            }
            else
            {
                result.Add(node.val.ToString());
                stack.Push(node.right);
                stack.Push(node.left);
            }
        }
        return string.Join(',', result);
    }

    // Decodes your encoded data to tree.
    public TreeNode Deserialize(string data)
    {
        string[] values = data.Split(',');
        int i = 0;
        return Dfs(values, ref i);
    }

    TreeNode Dfs(string[] values, ref int i)
    {
        i++;
        if (values[i] == "N")
        {
            return null;
        }
        var root = new TreeNode(int.Parse(values[i]))
        {
            left = Dfs(values, ref i),
            right = Dfs(values, ref i),
        };
        return root;
    }
}
