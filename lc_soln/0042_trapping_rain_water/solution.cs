public class Solution {
    public int Trap(int[] height) {
        int n = height.Length;
        if (n < 3) {
            return 0;
        }

        int l = 0;
        int r = n - 1;
        int lh = height[l];
        int rh = height[r];
        int result = 0;
        while (l < r) {
            if (lh < rh) {
                ++l;
                lh = int.Max(lh, height[l]);
                result += lh - height[l];
            } else {
                --r;
                rh = int.Max(rh, height[r]);
                result += rh - height[r];
            }
        }
        return result;
    }
}

