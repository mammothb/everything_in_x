public class Solution
{
    public bool SearchMatrix(int[][] matrix, int target)
    {
        int nc = matrix[0].Length;

        int row = -1;
        int t = 0;
        int b = matrix.Length - 1;
        while (t <= b)
        {
            int mid = t + (b - t) / 2;
            if (matrix[mid][0] <= target && target <= matrix[mid][nc - 1])
            {
                row = mid;
                break;
            }
            if (matrix[mid][nc - 1] < target)
            {
                t = mid + 1;
            }
            else
            {
                b = mid - 1;
            }
        }

        if (row == -1)
        {
            return false;
        }

        int l = 0;
        int r = nc - 1;
        while (l <= r)
        {
            int mid = l + (r - l) / 2;
            if (matrix[row][mid] == target)
            {
                return true;
            }
            if (matrix[row][mid] < target)
            {
                l = mid + 1;
            }
            else
            {
                r = mid - 1;
            }
        }

        return false;
    }
}
