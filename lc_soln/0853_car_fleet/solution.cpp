#include <vector>
#include <algorithm>

class Solution {
public:
    int carFleet(int target, vector<int>& position, vector<int>& speed) {
        int n = position.size();
        std::vector<std::pair<int, int>> pos_and_spd;
        pos_and_spd.reserve(n);
        for (int i = 0; i < position.size(); ++i) {
            pos_and_spd.emplace_back(position[i], speed[i]);
        }
        std::sort(pos_and_spd.begin(), pos_and_spd.end(),
                  [](std::pair<int, int>& a, std::pair<int, int>& b) { return a.first > b.first; });
        std::vector<double> result;
        for (const std::pair<int, int>& p : pos_and_spd) {
            double time = static_cast<double>(target - p.first) / static_cast<double>(p.second);
            if (!result.empty() && time <= result.back()) {
                continue;
            }
            result.push_back(time);
        }
        return result.size();
    }
};

