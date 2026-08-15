struct LRUCacheNode {
    int key = -1;
    int val = -1;
    LRUCacheNode* prev = nullptr;
    LRUCacheNode* next = nullptr;

    LRUCacheNode() {}

    LRUCacheNode(int key, int val)
        : key{key},
          val{val}
    {}
};
LRUCacheNode* removeNode(LRUCacheNode* node) {
    node->prev->next = node->next;
    node->next->prev = node->prev;
    node->prev = nullptr;
    node->next = nullptr;
    return node;
}

LRUCacheNode* insertAfter(LRUCacheNode* node, LRUCacheNode* target) {
    node->prev = target;
    node->next = target->next;
    target->next->prev = node;
    target->next = node;
    return node;
}

class LRUCache {
    int mCapacity;
    int mSize = 0;
    std::unordered_map<int, LRUCacheNode*> mKeyToNode;
    LRUCacheNode* mpHead;
    LRUCacheNode* mpTail;
public:
    LRUCache(int capacity) {
        mCapacity = capacity;
        mpHead = new LRUCacheNode();
        mpTail = new LRUCacheNode();
        mpHead->next = mpTail;
        mpTail->prev = mpHead;
    }

    int get(int key) {
        if (!mKeyToNode.contains(key)) {
            return -1;
        }
        LRUCacheNode* node = mKeyToNode[key];
        node = removeNode(node);
        node = insertAfter(node, mpHead);
        return node->val;
    }

    void put(int key, int value) {
        if (!mKeyToNode.contains(key)) {
            if (mSize == mCapacity) {
                LRUCacheNode* removed = removeNode(mpTail->prev);
                mKeyToNode.erase(removed->key);
                mSize--;
            }
            LRUCacheNode* node = new LRUCacheNode(key, value);
            node = insertAfter(node, mpHead);
            mKeyToNode[key] = node;
            mSize++;
        } else {
            LRUCacheNode* node = mKeyToNode[key];
            node = removeNode(node);
            node = insertAfter(node, mpHead);
            node->val = value;
        }
    }
};

int main() {
    return 0;
}
