class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        std::deque<int> q;
        std::vector<int> result;
        int start = 0;
        for (int i = 0; i < nums.size(); ++i) {
            while (!q.empty() && nums[q.back()] < nums[i]) {
                q.pop_back();
            }
            q.push_back(i);
            if (q.front() < start) {
                q.pop_front();
            }
            if (i - start + 1 >= k) {
                result.push_back(nums[q.front()]);
                start++;
            }
        }
        return result;
    }
};

int main() {
    return 0;
}
