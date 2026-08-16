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
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        if (lists.empty()) {
            return nullptr;
        }
        return split(lists, 0, lists.size() - 1);
    }

    ListNode* split(std::vector<ListNode*>& lists, int l, int r) {
        if (l > r) {
            return nullptr;
        }
        if (l == r) {
            return lists[l];
        }
        int mid = l + (r - l) / 2;
        ListNode* left = split(lists, l, mid);
        ListNode* right = split(lists, mid + 1, r);
        return merge(left, right);
    }

    ListNode* merge(ListNode* left, ListNode* right) {
        ListNode* dummy = new ListNode();
        ListNode* curr = dummy;
        while (left != nullptr && right != nullptr) {
            if (left->val < right->val) {
                curr->next = left;
                left = left->next;
            } else {
                curr->next = right;
                right = right->next;
            }
            curr = curr->next;
        }
        if (left != nullptr) {
            curr->next = left;
        }
        if (right != nullptr) {
            curr->next = right;
        }
        return dummy->next;
    }
};

int main() {
    return 0;
}
