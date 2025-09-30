class Solution {
public:
    vector<int> canSeePersonsCount(vector<int>& heights) {
        int n = heights.size();
        vector<int> result(n, 0);

        vector<int> stack;
        for (int i = n - 1; i >= 0; --i) {
            while (!stack.empty() && heights[stack.back()] <= heights[i]) {
                result[i]++;
                stack.pop_back();
            }
            if (!stack.empty()) {
                result[i]++;
            }
            stack.push_back(i);
        }
        return result;
    }
};

