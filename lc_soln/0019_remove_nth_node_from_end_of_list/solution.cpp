/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* fast = head;
        for (int i = 0; i < n; ++i) {
            fast = fast->next;
        }
        ListNode* prev = nullptr;
        ListNode* curr = head;
        while (fast != nullptr) {
            fast = fast->next;
            prev = curr;
            curr = curr->next;
        }
        if (curr == head) {
            return head->next;
        }
        prev->next = curr->next;
        return head;
    }
};

int main() {
    return 0;
}
