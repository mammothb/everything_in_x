class TimeMap {
    std::unordered_map<std::string, std::vector<std::pair<int, std::string>>> data;

public:
    TimeMap() : data{} {}

    void set(string key, string value, int timestamp) {
        data[key].emplace_back(timestamp, value);
    }

    string get(string key, int timestamp) {
        std::vector<std::pair<int, std::string>> values = data[key];
        int l = 0;
        int r = values.size() - 1;
        int idx = -1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (values[mid].first <= timestamp) {
                idx = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        return idx == -1 ? "" : values[idx].second;
    }
};

int main() {
    return 0;
}
