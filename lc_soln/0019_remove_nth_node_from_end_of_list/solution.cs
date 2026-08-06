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
    public ListNode RemoveNthFromEnd(ListNode head, int n)
    {
        var fast = head;
        while (n > 0)
        {
            fast = fast.next;
            n--;
        }
        ListNode prev = null;
        ListNode curr = head;
        while (fast is not null)
        {
            fast = fast.next;
            prev = curr;
            curr = curr.next;
        }
        if (curr == head)
        {
            return head.next;
        }
        prev.next = curr.next;
        return head;
    }
}
