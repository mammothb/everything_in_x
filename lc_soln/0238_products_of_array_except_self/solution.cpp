class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        std::vector<int> fwd(n);
        std::vector<int> bwd(n);
        fwd[0] = 1;
        bwd[n - 1] = 1;
        for (int i = 1; i < nums.size(); ++i) {
            fwd[i] = fwd[i - 1] * nums[i - 1];
            bwd[n - 1 - i] = bwd[n - i] * nums[n - i];
        }
        std::vector<int> result(n);
        for (int i = 0; i < nums.size(); ++i) {
            result[i] = fwd[i] * bwd[i];
        }
        return result;
    }
};

int main() {
    return 0;
}
