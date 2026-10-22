class Solution {
   public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        std::ranges::sort(nums);
        std::vector<std::vector<int>> result;
        std::vector<int> curr;
        dfs(nums, 0, nums.size(), curr, result);
        return result;
    }

    void dfs(const std::vector<int>& nums, int start, int stop, std::vector<int>& curr,
             std::vector<std::vector<int>>& result) {
        result.push_back(curr);
        for (int i = start; i < stop; i++) {
            if (i > start && nums[i] == nums[i - 1]) {
                continue;
            }
            curr.push_back(nums[i]);
            dfs(nums, i + 1, stop, curr, result);
            curr.pop_back();
        }
    }
};

int main() {
    return 0;
}
