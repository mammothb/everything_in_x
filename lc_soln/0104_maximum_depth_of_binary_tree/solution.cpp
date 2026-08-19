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
    int maxDepth(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        int result = 0;
        std::deque<TreeNode*> q;
        q.push_back(root);
        while (!q.empty()) {
            int n = q.size();
            while (n > 0) {
                TreeNode* node = q.front();
                q.pop_front();
                if (node->left != nullptr) {
                    q.push_back(node->left);
                }
                if (node->right != nullptr) {
                    q.push_back(node->right);
                }
                n--;
            }
            result++;
        }
        return result;
    }
};

int main() {
    return 0;
}
