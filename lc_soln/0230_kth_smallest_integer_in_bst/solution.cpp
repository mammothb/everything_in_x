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
    int kthSmallest(TreeNode* root, int k) {
        int result = -1;
        dfs(root, k, result);
        return result;
    }

    void dfs(TreeNode* root, int& k, int& result) {
        if (root == nullptr) {
            return;
        }
        dfs(root->left, k, result);
        if (k == 0) {
            return;
        }
        k--;
        if (k == 0) {
            result = root->val;
            return;
        }
        dfs(root->right, k, result);
    }
};

int main() {
    return 0;
}
