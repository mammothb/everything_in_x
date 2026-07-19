class MinStack {
private:
    std::vector<int> data;
    std::vector<int> mins;
public:
    MinStack() {}

    void push(int val) {
        data.push_back(val);
        if (mins.empty()) {
            mins.push_back(val);
        } else {
            mins.push_back(std::min(getMin(), val));
        }
    }

    void pop() {
        data.pop_back();
        mins.pop_back();
    }

    int top() {
        return data.back();
    }

    int getMin() {
        return mins.back();
    }
};

int main() {
    return 0;
}
