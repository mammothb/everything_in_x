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
    public ListNode ReverseKGroup(ListNode head, int k)
    {
        ListNode dummy = new ListNode(next: head);
        ListNode groupPrev = dummy;
        while (true)
        {
            ListNode kth = GetKth(groupPrev, k);
            if (kth is null)
            {
                break;
            }
            ListNode groupNext = kth.next;

            ListNode prev = kth.next;
            ListNode curr = groupPrev.next;
            while (curr != groupNext)
            {
                ListNode tmp = curr.next;
                curr.next = prev;
                prev = curr;
                curr = tmp;
            }

            ListNode tmp2 = groupPrev.next;
            groupPrev.next = kth;
            groupPrev = tmp2;
        }
        return dummy.next;
    }

    ListNode GetKth(ListNode curr, int k)
    {
        while (curr is not null && k > 0)
        {
            curr = curr.next;
            k--;
        }
        return curr;
    }
}
