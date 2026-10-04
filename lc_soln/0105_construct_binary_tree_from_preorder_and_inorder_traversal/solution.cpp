/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

class Solution {
   public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        std::unordered_map<int, int> in_to_idx;
        for (int i = 0; i < inorder.size(); ++i) {
            in_to_idx[inorder[i]] = i;
        }
        int pre_idx = 0;
        return build(preorder, inorder, 0, inorder.size() - 1, pre_idx, in_to_idx);
    }

    TreeNode* build(const std::vector<int>& preorder, const std::vector<int>& inorder, int in_start,
                    int in_stop, int& pre_idx, const std::unordered_map<int, int>& in_to_idx) {
        if (in_start > in_stop) {
            return nullptr;
        }
        TreeNode* root = new TreeNode(preorder[pre_idx]);
        pre_idx++;
        root->left =
            build(preorder, inorder, in_start, in_to_idx.at(root->val) - 1, pre_idx, in_to_idx);
        root->right =
            build(preorder, inorder, in_to_idx.at(root->val) + 1, in_stop, pre_idx, in_to_idx);
        return root;
    }
};

int main() {
    return 0;
}
