class Solution {
public:
    vector<string> sortFeatures(vector<string> &features, vector<string> &responses) {
        unordered_map<string, int> popularity;
        for (const string& feature : features) {
            popularity[feature]++;
        }

        for (const string& response : responses) {
            unordered_set<string> seen;
            istringstream iss(response);
            string token;
            while (iss >> token) {
                if (!token.empty() && seen.count(token) == 0) {
                    popularity[token]++;
                    seen.insert(token);
                }
            }
        }
        stable_sort(features.begin(), features.end(), [popularity](const string& a, const string& b) {
            return popularity.at(a) > popularity.at(b);
        });
        return features;
    }
};
