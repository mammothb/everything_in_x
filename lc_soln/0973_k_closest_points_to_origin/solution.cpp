class Solution {
   public:
    vector<vector<int>> kClosest(vector<vector<int>>& points, int k) {
        std::priority_queue<std::pair<int, std::pair<int, int>>> h;
        for (const auto& point : points) {
            int dist = point[0] * point[0] + point[1] * point[1];
            h.push({dist, {point[0], point[1]}});
            if (h.size() > k) {
                h.pop();
            }
        }
        std::vector<std::vector<int>> result;
        while (!h.empty()) {
            result.push_back({h.top().second.first, h.top().second.second});
            h.pop();
        }
        return result;
    }
};

int main() {
    return 0;
}
