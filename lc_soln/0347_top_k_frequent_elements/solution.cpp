class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        std::unordered_map<int, int> counter;
        for (int num : nums) {
            counter[num]++;
        }
        std::vector<std::vector<int>> buckets(nums.size() + 1, std::vector<int>());
        for (const auto& [num, freq] : counter) {
            buckets[freq].push_back(num);
        }
        std::vector<int> result;
        result.reserve(k);
        for (int i = buckets.size() - 1; i >= 0; --i) {
            for (int num : buckets[i]) {
                result.push_back(num);
                if (result.size() == k) {
                    return result;
                }
            }
        }
        return result;
    }
};

int main() {
    return 0;
}
