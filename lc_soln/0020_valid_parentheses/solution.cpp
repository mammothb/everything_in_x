class Solution {
public:
    bool isValid(string s) {
        vector<char> stack;
        for (char c : s) {
            if (c == '(' || c == '[' || c == '{') {
                stack.push_back(c);
                continue;
            }
            if (stack.empty()) {
                return false;
            }
            char c2 = stack.back();
            if (!((c2 == '(' && c == ')') || (c2 == '[' && c == ']') ||
                  (c2 == '{' && c == '}'))) {
                return false;
            }
            stack.pop_back();
        }
        return stack.empty();
    }
};

