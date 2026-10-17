class Solution {
   public:
    vector<vector<int>> combinationSum(vector<int>& nums, int target) {
        std::ranges::sort(nums);
        std::vector<std::vector<int>> result;
        std::vector<int> buf;
        dfs(nums, 0, nums.size(), 0, buf, result, target);
        return result;
    }

    void dfs(const std::vector<int>& nums, int start, int stop, int curr, std::vector<int>& buf,
             std::vector<std::vector<int>>& result, int target) {
        if (curr == target) {
            result.push_back(buf);
            return;
        }
        if (start >= stop) {
            return;
        }
        for (int i = start; i < stop; i++) {
            curr += nums[i];
            if (curr > target) {
                return;
            }
            buf.push_back(nums[i]);
            dfs(nums, i, stop, curr, buf, result, target);
            buf.pop_back();
            curr -= nums[i];
        }
    }
};

int main() {
    return 0;
}
