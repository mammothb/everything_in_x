class Solution {
public:
    int minEatingSpeed(vector<int>& piles, int h) {
        int l = 1;
        int r = *std::max_element(piles.begin(), piles.end());
        int result = r;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            int count = 0;
            for (const int pile : piles) {
                count += (pile + mid - 1) / mid;
            }
            if (count <= h) {
                result = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return result;
    }
};

int main() {
    return 0;
}
