public class Solution
{
    public int LargestRectangleArea(int[] heights)
    {
        int[] lefts = new int[heights.Length];
        Array.Fill(lefts, -1);
        var stack = new Stack<int>();
        for (int i = 0; i < heights.Length; ++i)
        {
            while (stack.Count > 0 && heights[stack.Peek()] >= heights[i])
            {
                stack.Pop();
            }
            if (stack.Count > 0)
            {
                lefts[i] = stack.Peek();
            }
            stack.Push(i);
        }

        int[] rights = new int[heights.Length];
        Array.Fill(rights, heights.Length);
        stack.Clear();
        for (int i = heights.Length - 1; i >= 0; --i)
        {
            while (stack.Count > 0 && heights[stack.Peek()] >= heights[i])
            {
                stack.Pop();
            }
            if (stack.Count > 0)
            {
                rights[i] = stack.Peek();
            }
            stack.Push(i);
        }

        int result = 0;
        for (int i = 0; i < heights.Length; ++i)
        {
            result = int.Max(result, heights[i] * (rights[i] - lefts[i] - 1));
        }
        return result;
    }
}
