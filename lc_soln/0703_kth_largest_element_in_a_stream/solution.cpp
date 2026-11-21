class KthLargest {
    std::priority_queue<int, std::vector<int>, std::greater<int>> q_;
    int k_;

   public:
    KthLargest(int k, vector<int>& nums) : k_(k), q_(std::greater<int>(), nums) {
        while (q_.size() > k_) {
            q_.pop();
        }
    }

    int add(int val) {
        q_.push(val);
        if (q_.size() > k_) {
            q_.pop();
        }
        return q_.top();
    }
};

int main() {
    return 0;
}
