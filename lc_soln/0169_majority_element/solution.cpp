#include <vector>

using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int count = 1;
        int last = nums[0];
        for (int i = 1; i < nums.size(); ++i) {
            int num = nums[i];
            if (num == last) {
                ++count;
            } else {
                --count;
                if (count == 0) {
                    last = num;
                    count = 1;
                }
            }
        }
        return last;
    }
};

