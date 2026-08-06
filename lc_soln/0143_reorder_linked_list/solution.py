# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        fast = head.next
        slow = head
        while fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next

        second = slow.next
        slow.next = None
        prev = None
        while second is not None:
            tmp = second.next
            second.next = prev
            prev = second
            second = tmp
        first = head
        second = prev
        while first is not None and second is not None:
            tmp1 = first.next
            tmp2 = second.next
            first.next = second
            second.next = tmp1
            first = tmp1
            second = tmp2


def main(): ...


if __name__ == "__main__":
    main()
