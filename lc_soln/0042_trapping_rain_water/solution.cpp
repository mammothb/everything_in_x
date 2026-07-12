class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
        if (n < 3) {
            return 0;
        }

        int l = 0;
        int r = n - 1;
        int lh = height[l];
        int rh = height[r];
        int result = 0;
        int curr;
        while (l < r) {
            if (lh < rh) {
                ++l;
                curr = height[l];
                lh = std::max(lh, curr);
            } else {
                --r;
                curr = height[r];
                rh = std::max(rh, curr);
            }
            result += max(0, min(lh, rh) - curr);
        }
        return result;
    }
};

int main() {
    return 0;
}
