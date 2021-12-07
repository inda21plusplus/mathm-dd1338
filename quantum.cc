#include <algorithm>
#include <queue>
#include <unordered_set>
#include <unordered_map>
#include <iostream>
#include <iomanip>

using namespace std;

struct op_t {
public:
    int set, clear, flip, cost;

    int apply(int val) {
        val |= set;
        val &= ~clear;
        val ^= flip;
        return val;
    }

    op_t() {}

    op_t(const string& s, int cost): set(0), clear(0), flip(0), cost(cost) {
        for (size_t i = 0; i < s.size(); ++i) {
            int *target = 0;
            switch (s[s.size() - 1 - i]) {
                case 'S': target = &set;   break;
                case 'C': target = &clear; break;
                case 'F': target = &flip;  break;
            }
            if (target) *target |= 1 << i;
        }
    }
};



int main(void) {
    int n; cin >> n;
    for (int i = 0; i < n; ++i) {
        int len, n_op, n_w; cin >> len >> n_op >> n_w;
        vector<op_t> ops(n_op);
        string op_str;
        for (int j = 0; j < n_op; ++j) {
            int cost;
            cin >> op_str >> cost;
            ops[j] = op_t(op_str, cost);
        }

        for (int j = 0; j < n_w; ++j) {
            string from_str, to_str; cin >> from_str >> to_str;
            int from = 0;
            for (int k = 0; k < len; ++k)
                from += (from_str[len - 1 - k] == '1') << k;
            int to = 0;
            for (int k = 0; k < len; ++k)
                to += (to_str[len - 1 - k] == '1') << k;

            priority_queue<pair<int, int>> q;
            q.emplace(0, from);
            unordered_map<int, int> visited;
            bool found = false;
            while (!q.empty()) {
                auto [cost, val] = q.top(); q.pop();
                if (val == to) {
                    cout << -cost << ' ';
                    found = true;
                    break;
                }
                for (auto& op : ops) {
                    int next = op.apply(val);
                    int n_cost = cost - op.cost;
                    auto it = visited.find(next);
                    if (it == visited.end() || it->second < n_cost) {
                        visited[next] = n_cost;
                        q.emplace(n_cost, next);
                    }
                }
            }
            if (!found) {
                cout << "NP ";
            }
        }
        cout << endl;
    }
}
