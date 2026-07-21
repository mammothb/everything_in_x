class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        std::vector<int> result(temperatures.size(), 0);
        std::stack<int> stack;
        for (int i = 0; i < temperatures.size(); ++i) {
            while (!stack.empty() && temperatures[i] > temperatures[stack.top()]) {
                int j = stack.top();
                result[j] = i - j;
                stack.pop();
            }
            stack.push(i);
        }
        return result;
    }
};

int main() {
    return 0;
}
