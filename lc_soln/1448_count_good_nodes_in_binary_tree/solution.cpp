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
    int goodNodes(TreeNode* root) {
        int result = 0;
        std::stack<TreeNode*> node_stack;
        std::stack<int> val_stack;
        node_stack.push(root);
        val_stack.push(root->val);
        while (!node_stack.empty()) {
            TreeNode* node = node_stack.top();
            int max_val = std::max(val_stack.top(), node->val);
            node_stack.pop();
            val_stack.pop();
            if (node->val >= max_val) {
                result++;
            }
            if (node->left != nullptr) {
                node_stack.push(node->left);
                val_stack.push(max_val);
            }
            if (node->right != nullptr) {
                node_stack.push(node->right);
                val_stack.push(max_val);
            }
        }
        return result;
    }
};

int main() {
    return 0;
}
