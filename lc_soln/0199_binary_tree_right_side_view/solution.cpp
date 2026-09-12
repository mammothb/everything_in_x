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
    vector<int> rightSideView(TreeNode* root) {
        std::vector<int> result;
        if (root == nullptr) {
            return result;
        }
        std::queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            int n = q.size();
            while (n-- > 0) {
                TreeNode* node = q.front();
                q.pop();
                if (n == 0) {
                    result.push_back(node->val);
                }
                if (node->left != nullptr) {
                    q.push(node->left);
                }
                if (node->right != nullptr) {
                    q.push(node->right);
                }
            }
        }
        return result;
    }
};

int main() {
    return 0;
}
