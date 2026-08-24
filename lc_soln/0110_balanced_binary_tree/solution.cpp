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
    bool isBalanced(TreeNode* root) {
        int height;
        return dfs(root, height);
    }

    bool dfs(TreeNode* root, int& height) {
        if (root == nullptr) {
            height = 0;
            return true;
        }
        int left_height;
        bool left = dfs(root->left, left_height);
        int right_height;
        bool right = dfs(root->right, right_height);
        height = 1 + max(left_height, right_height);
        return left && right && std::abs(left_height - right_height) < 2;
    }
};

int main() {
    return 0;
}
