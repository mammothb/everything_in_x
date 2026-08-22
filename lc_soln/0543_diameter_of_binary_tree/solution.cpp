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
    int diameterOfBinaryTree(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        return std::max(
            height(root->left) + height(root->right),
            std::max(diameterOfBinaryTree(root->left), diameterOfBinaryTree(root->right)));
    }

    int height(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        return 1 + std::max(height(root->left), height(root->right));
    }
};

int main() {
    return 0;
}
