// https://open.kattis.com/problems/tsp

#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <bitset>
#include <cstdint>
#include <cmath>

using namespace std;

using bs = bitset<1000>;

struct pair_hash {
    size_t operator()(const pair<bs, int>& p) const {
        auto h1 = hash<bitset<1000>>{}(p.first);
        auto h2 = hash<int>{}(p.second);
        return h1 ^ h2;
    }
};

unordered_map<pair<bs, int>, pair<uint64_t, vector<int>>, pair_hash> dp;

pair<uint64_t, vector<int>> tsp(int n, const bs& been, int k, const vector<vector<uint64_t>>& dist) {
    auto it = dp.find({ been, k });
    if (it != dp.end()) {
        return it->second;
    }

    uint64_t best = -1; // max uint
    vector<int> path;
    bs been_cpy = been;
    for (int m = 0; m < n; ++m) {
        if (!been[m]) continue;
        if (m == k) continue;
        been_cpy[m] = false;
        auto ans = tsp(n, been_cpy, m, dist);
        been_cpy[m] = true;
        uint64_t d = ans.first + dist[m][k];
        vector<int> p = ans.second; p.push_back(m);
        if (d < best) {
            best = d;
            path = p;
        }
    }

    dp[{ been, k }] = { best, path };

    return { best, path };
}

int main(void) {
    int n; cin >> n;

    vector<float> xs(n);
    vector<float> ys(n);

    for (int i = 0; i < n; ++i)
        cin >> xs[i] >> ys[i];

    vector<vector<uint64_t>> dist(n, vector<uint64_t>(n));

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            float dx = xs[i] - xs[j];
            float dy = ys[i] - ys[j];
            dist[i][j] = lround(dx * dx + dy * dy);
        }
    }

    bs all; all = ~all;

    uint64_t best;
    vector<int> path;
    for (int k = 0; k < n; ++k) {
        pair<uint64_t, vector<int>> ans = tsp(n, all, k, dist);
        if (ans.first < best) {
            best = ans.first;
            path = ans.second;
        }
    }

    for (int node : path) {
        cout << node << endl;
    }

    cerr << endl << best << endl;
}
