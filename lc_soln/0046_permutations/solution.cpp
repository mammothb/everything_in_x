class Solution {
   public:
    vector<vector<int>> permute(vector<int>& nums) {
        std::vector<std::vector<int>> result;
        dfs(nums, 0, nums.size(), result);
        return result;
    }

    void dfs(std::vector<int>& nums, int start, int stop, std::vector<std::vector<int>>& result) {
        if (start == stop) {
            result.push_back(nums);
            return;
        }
        for (int i = start; i < stop; i++) {
            std::swap(nums[i], nums[start]);
            dfs(nums, start + 1, stop, result);
            std::swap(nums[i], nums[start]);
        }
    }
};

int main() {
    return 0;
}
