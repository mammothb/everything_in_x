class Solution {
   public:
    vector<vector<int>> subsets(vector<int>& nums) {
        std::vector<std::vector<int>> result;
        std::vector<int> curr;
        result.push_back(curr);
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            dfs(nums, i, n, curr, result);
        }
        return result;
    }

    void dfs(const std::vector<int>& nums, int start, int stop, std::vector<int> curr,
             std::vector<std::vector<int>>& result) {
        if (start >= stop) {
            return;
        }
        curr.push_back(nums[start]);
        result.push_back(curr);
        for (int i = start + 1; i < stop; i++) {
            dfs(nums, i, stop, curr, result);
        }
        curr.pop_back();
    }
};

int main() {
    return 0;
}
