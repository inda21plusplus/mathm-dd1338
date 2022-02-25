#include "interval-cover.h"

#include <algorithm>
#include <cassert>
#include <vector>
#include <iostream>

using namespace std;

struct Case {
    vector<pair<float, float>> intervals;
    pair<float, float> to_cover;
    vector<size_t> expected;
};

int main() {
    vector<Case> cases = {
        {
            { make_pair(-0.9, -0.1), make_pair(-0.2, 2), make_pair(-0.7, 1) },
            make_pair(-0.5f, 1.0f),
            { 1 },
        },
        {
            { make_pair(0, 0.25), make_pair(0.25, 0.75), make_pair(0.75, 0.999) },
            make_pair(0.f, 1.f),
            { },
        },
        {
            { make_pair(0, 0.25), make_pair(0.25, 0.75), make_pair(0.75, 1) },
            make_pair(0.f, 1.f),
            { 0, 1, 2 },
        },
        {
            { make_pair(1, 1) },
            make_pair(1.f, 1.f),
            { 0 },
        },
    };

    for (size_t i = 0; i < cases.size(); ++i) {
        Case &c = cases[i];
        vector<size_t> indices = cover_interval(c.intervals, c.to_cover.first, c.to_cover.second);
        assert(indices.size() == c.expected.size());
        for (size_t i = 0; i < indices.size(); ++i) {
            assert(indices[i] == c.expected[i]);
        }
    }
    cout << "Great Success!\n";
}
