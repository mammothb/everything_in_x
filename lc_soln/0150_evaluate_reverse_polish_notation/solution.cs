public class Solution
{
    public int EvalRPN(string[] tokens)
    {
        var stack = new Stack<int>();
        foreach (string c in tokens)
        {
            if (c == "+")
            {
                int n2 = stack.Pop();
                int n1 = stack.Pop();
                stack.Push(n1 + n2);
            }
            else if (c == "-")
            {
                int n2 = stack.Pop();
                int n1 = stack.Pop();
                stack.Push(n1 - n2);
            }
            else if (c == "*")
            {
                int n2 = stack.Pop();
                int n1 = stack.Pop();
                stack.Push(n1 * n2);
            }
            else if (c == "/")
            {
                int n2 = stack.Pop();
                int n1 = stack.Pop();
                stack.Push(n1 / n2);
            }
            else
            {
                stack.Push(int.Parse(c));
            }
        }
        return stack.Peek();
    }
}
