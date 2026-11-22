public class Solution
{
    public int LastStoneWeight(int[] stones)
    {
        var h = new PriorityQueue<int, int>(stones.Length);
        foreach (int stone in stones)
        {
            h.Enqueue(stone, -stone);
        }
        while (h.Count > 1)
        {
            int x = h.Dequeue();
            int y = h.Dequeue();
            if (x != y)
            {
                h.Enqueue(x - y, y - x);
            }
        }
        return h.Count == 0 ? 0 : h.Peek();
    }
}
