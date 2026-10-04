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
    public TreeNode BuildTree(int[] preorder, int[] inorder)
    {
        var inToIdx = new Dictionary<int, int>(inorder.Length);
        for (int i = 0; i < inorder.Length; i++)
        {
            inToIdx[inorder[i]] = i;
        }
        int preIdx = 0;
        return Build(preorder, 0, inorder.Length - 1, ref preIdx, inToIdx);
    }

    TreeNode Build(
        int[] preorder,
        int inStart,
        int inStop,
        ref int preIdx,
        Dictionary<int, int> inToIdx
    )
    {
        if (inStart > inStop)
        {
            return null;
        }
        var root = new TreeNode(preorder[preIdx]);
        preIdx++;
        if (inToIdx.TryGetValue(root.val, out int idx))
        {
            root.left = Build(preorder, inStart, idx - 1, ref preIdx, inToIdx);
            root.right = Build(preorder, idx + 1, inStop, ref preIdx, inToIdx);
        }
        return root;
    }
}
