public class Solution
{
    public int CarFleet(int target, int[] position, int[] speed)
    {
        (int, int)[] posAndSpd = position.Zip(speed, (pos, spd) => (pos, spd)).ToArray();
        Array.Sort(
            posAndSpd,
            ((int pos, int spd) a, (int pos, int spd) b) => b.pos.CompareTo(a.pos)
        );
        var stack = new Stack<double>();
        foreach ((int pos, int spd) in posAndSpd)
        {
            double time = (double)(target - pos) / (double)spd;
            if (stack.Count > 0 && time <= stack.Peek())
            {
                continue;
            }
            stack.Push(time);
        }
        return stack.Count;
    }
}
