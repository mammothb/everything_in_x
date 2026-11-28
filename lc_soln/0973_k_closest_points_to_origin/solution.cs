public class Solution
{
    public int[][] KClosest(int[][] points, int k)
    {
        var h = new PriorityQueue<int[], int>();
        foreach (int[] point in points)
        {
            if (h.Count == k)
            {
                h.EnqueueDequeue(point, -(point[0] * point[0] + point[1] * point[1]));
            }
            else
            {
                h.Enqueue(point, -(point[0] * point[0] + point[1] * point[1]));
            }
        }
        int[][] result = new int[k][];
        int i = 0;
        while (h.Count > 0)
        {
            result[i] = h.Dequeue();
            i++;
        }

        return result;
    }
}
