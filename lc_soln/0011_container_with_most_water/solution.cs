public class Solution {
    public int MaxArea(int[] heights) {
        int result = -1;
        int l = 0;
        int r = heights.Length - 1;
        while (l < r) {
            int h = int.Min(heights[l], heights[r]);
            int curr = (r - l) * h;
            result = int.Max(result, curr);
            if (heights[l] < heights[r]) {
                l++;
            } else {
                r--;
            }
        }
        return result;
    }
}

