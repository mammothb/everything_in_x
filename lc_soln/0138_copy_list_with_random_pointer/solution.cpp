/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) {
            return nullptr;
        }
        Node* new_head = new Node(head->val);
        std::unordered_map<Node*, Node*> old_to_new;
        old_to_new[head] = new_head;

        Node* curr = head;
        Node* new_curr = new_head;
        while (curr != nullptr) {
            if (curr->next != nullptr) {
                Node* new_next;
                if (old_to_new.contains(curr->next)) {
                    new_next = old_to_new[curr->next];
                } else {
                    new_next = new Node(curr->next->val);
                    old_to_new[curr->next] = new_next;
                }
                new_curr->next = new_next;
            }
            if (curr->random != nullptr) {
                Node* new_random;
                if (old_to_new.contains(curr->random)) {
                    new_random = old_to_new[curr->random];
                } else {
                    new_random = new Node(curr->random->val);
                    old_to_new[curr->random] = new_random;
                }
                new_curr->random = new_random;
            }

            curr = curr->next;
            new_curr = new_curr->next;
        }
        return new_head;
    }
};

int main() {
    return 0;
}
