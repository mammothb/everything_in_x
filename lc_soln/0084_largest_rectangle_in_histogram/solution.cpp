class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        std::vector<int> lefts(heights.size(), -1);
        std::stack<int> stack;
        for (int i = 0; i < heights.size(); ++i) {
            while (!stack.empty() && heights[stack.top()] >= heights[i]) {
                stack.pop();
            }
            if (!stack.empty()) {
                lefts[i] = stack.top();
            }
            stack.push(i);
        }
        std::vector<int> rights(heights.size(), heights.size());
        std::stack<int>().swap(stack);
        for (int i = heights.size() - 1; i >= 0; --i) {
            while (!stack.empty() && heights[stack.top()] >= heights[i]) {
                stack.pop();
            }
            if (!stack.empty()) {
                rights[i] = stack.top();
            }
            stack.push(i);
        }
        int result = 0;
        for (int i = 0; i < heights.size(); ++i) {
            result = std::max(result, heights[i] * (rights[i] - lefts[i] - 1));
        }
        return result;
    }
};

int main() {
    return 0;
}
