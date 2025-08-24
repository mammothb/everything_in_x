#include <algorithm>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    int maxEvents(vector<vector<int>>& events) {
        ranges::sort(events, [](const vector<int>& a, const vector<int>& b) {
            return a[0] < b[0];
        });
        priority_queue<int, vector<int>, greater<int>> pq;
        int result = 0;
        int i = 0;
        int n = events.size();
        int day = events[0][0];
        while (i < n || !pq.empty()) {
            while (!pq.empty() && pq.top() < day) {
                pq.pop();
            }
            while (i < n && events[i][0] == day) {
                pq.push(events[i][1]);
                ++i;
            }
            if (!pq.empty()) {
                pq.pop();
                ++result;
            }
            ++day;
        }
        return result;
    }
};

