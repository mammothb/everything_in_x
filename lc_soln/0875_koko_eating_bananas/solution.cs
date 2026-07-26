public class Solution
{
    public int MinEatingSpeed(int[] piles, int h)
    {
        int l = 1;
        int r = piles.Max();
        int result = r;
        while (l <= r)
        {
            int mid = l + (r - l) / 2;
            int count = 0;
            foreach (int pile in piles)
            {
                count += (pile + mid - 1) / mid;
            }
            if (count <= h)
            {
                result = mid;
                r = mid - 1;
            }
            else
            {
                l = mid + 1;
            }
        }
        return result;
    }
}
