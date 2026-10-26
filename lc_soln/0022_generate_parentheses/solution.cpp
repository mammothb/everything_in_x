class Solution {
public:
    vector<string> generateParenthesis(int n) {
        std::vector<std::string> result;
        std::vector<char> curr;
        dfs(0, n, curr, result);
        return result;
    }

    void dfs(int left, int right, std::vector<char>& curr, std::vector<std::string>& result) {
        if (left == 0 && right == 0) {
            result.push_back(std::string(curr.begin(), curr.end()));
            return;
        }
        if (left > 0) {
            curr.push_back(')');
            dfs(left - 1, right, curr, result);
            curr.pop_back();
        }
        if (right > 0) {
            curr.push_back('(');
            dfs(left + 1, right - 1, curr, result);
            curr.pop_back();
        }
    }
};

int main() {
    return 0;
}
