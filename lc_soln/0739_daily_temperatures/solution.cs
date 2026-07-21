public class Solution
{
    public int[] DailyTemperatures(int[] temperatures)
    {
        var result = new int[temperatures.Length];
        var stack = new Stack<(int temp, int idx)>();
        for (int i = 0; i < temperatures.Length; ++i)
        {
            int currTemp = temperatures[i];
            while (stack.Count > 0 && currTemp > stack.Peek().temp)
            {
                (int temp, int j) = stack.Pop();
                result[j] = i - j;
            }
            stack.Push((currTemp, i));
        }
        return result;
    }
}
