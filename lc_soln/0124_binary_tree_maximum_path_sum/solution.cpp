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
    int maxPathSum(TreeNode* root) {
        int result = INT_MIN;
        dfs(root, result);
        return result;
    }

    int dfs(TreeNode* root, int& result) {
        if (root == nullptr) {
            return 0;
        }

        int left_tree = dfs(root->left, result);
        int right_tree = dfs(root->right, result);

        int max_single = max(root->val, root->val + max(left_tree, right_tree));
        int max_sum = max(root->val + left_tree + right_tree, max_single);
        result = max(result, max_sum);

        return max_single;
    }
};

int main() {
    return 0;
}
