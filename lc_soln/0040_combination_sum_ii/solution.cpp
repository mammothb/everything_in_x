class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        std::ranges::sort(candidates);
        std::vector<std::vector<int>> result;
        std::vector<int> curr;
        dfs(candidates, 0, candidates.size(), target, curr, result);
        return result;
    }

    void dfs(const std::vector<int>& nums, int start, int stop, int target, std::vector<int>& curr, std::vector<std::vector<int>>& result) {
        if (target == 0) {
            result.push_back(curr);
            return;
        }
        if (start >= stop) {
            return;
        }
        for (int i = start; i < stop; i++) {
            if (i > start && nums[i] == nums[i - 1]) {
                continue;
            }
            target -= nums[i];
            if (target < 0) {
                return;
            }
            curr.push_back(nums[i]);
            dfs(nums, i + 1, stop, target, curr, result);
            curr.pop_back();
            target += nums[i];
        }
    }
};

int main() {
    return 0;
}
