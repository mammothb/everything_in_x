public class Solution
{
    public int MaxProfit(int[] prices)
    {
        int result = 0;
        int buy = int.MaxValue;
        foreach (int price in prices)
        {
            if (price < buy)
            {
                buy = price;
            }
            else
            {
                result = int.Max(result, price - buy);
            }
        }
        return result;
    }
}

