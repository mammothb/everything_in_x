public class MinStack
{
    private Stack<int> data;
    private Stack<int> mins;

    public MinStack()
    {
        data = new();
        mins = new();
    }

    public void Push(int val)
    {
        data.Push(val);
        mins.Push(mins.Count == 0 ? val : int.Min(GetMin(), val));
    }

    public void Pop()
    {
        data.Pop();
        mins.Pop();
    }

    public int Top()
    {
        return data.Peek();
    }

    public int GetMin()
    {
        return mins.Peek();
    }
}
