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
#include <string_view>
#include <ranges>

class Codec {
   public:
    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        std::ostringstream oss;
        std::stack<TreeNode*> stack;
        stack.push(root);
        while (!stack.empty()) {
            oss << ",";
            TreeNode* node = stack.top();
            stack.pop();
            if (node == nullptr) {
                oss << "N";
            } else {
                oss << std::to_string(node->val);
                stack.push(node->right);
                stack.push(node->left);
            }
        }
        return oss.str();
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        auto tokens =
            data | std::views::split(',') | std::views::transform([](auto&& subrange) {
            return std::string(subrange.begin(), subrange.end());
        });
        std::vector<std::string> values(tokens.begin(), tokens.end());
        int i = 0;
        return dfs(values, i);
    }

    TreeNode* dfs(const std::vector<std::string>& values, int& i) {
        i++;
        if (values[i] == "N") {
            return nullptr;
        }
        TreeNode* node = new TreeNode(std::stoi(values[i]));
        node->left = dfs(values, i);
        node->right = dfs(values, i);
        return node;
    }
};

int main() {
    return 0;
}
