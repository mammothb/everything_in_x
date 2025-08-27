#include <string>
#include <string_view>
#include <vector>

using namespace std;

class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        vector<int> stack;
        for (const string& c : tokens) {
            if (c == "+" || c == "-" || c == "*" || c == "/") {
                eval(stack, c);
            } else {
                stack.push_back(stoi(c));
            }
        }
        return stack.back();
    }

    void eval(vector<int>& stack, string_view op) {
        int right = stack.back();
        stack.pop_back();
        int left = stack.back();
        stack.pop_back();
        if (op == "+") {
            stack.push_back(left + right);
        } else if (op == "-") {
            stack.push_back(left - right);
        } else if (op == "*") {
            stack.push_back(left * right);
        } else if (op == "/") {
            stack.push_back(left / right);
        }
    }
};

