/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

public class Solution
{
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
    {
        var dummy = new ListNode();
        ListNode curr = dummy;
        int total = 0;
        while (l1 != null || l2 != null || total > 0)
        {
            if (l1 != null)
            {
                total += l1.val;
                l1 = l1.next;
            }
            if (l2 != null)
            {
                total += l2.val;
                l2 = l2.next;
            }
            var node = new ListNode(total % 10);
            total /= 10;
            curr.next = node;
            curr = curr.next;
        }
        return dummy.next;
    }
}
