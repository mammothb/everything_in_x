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
    vector<vector<int>> levelOrder(TreeNode* root) {
        std::vector<std::vector<int>> result;
        if (root == nullptr) {
            return result;
        }
        std::deque<TreeNode*> q;
        q.push_back(root);
        while (!q.empty()) {
            int n = q.size();
            std::vector<int> row;
            while (n-- > 0) {
                TreeNode* node = q.front();
                q.pop_front();
                row.push_back(node->val);
                if (node->left != nullptr) {
                    q.push_back(node->left);
                }
                if (node->right != nullptr) {
                    q.push_back(node->right);
                }
            }
            result.push_back(row);
        }
        return result;
    }
};

int main() {
    return 0;
}
