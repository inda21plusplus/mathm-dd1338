#include "interval-cover.h"

#include <algorithm>
#include <cassert>

using namespace std;

int main() {
    vector<pair<float, float>> intervals = {
        make_pair(-0.9, -0.1),
        make_pair(-0.2, 2),
        make_pair(-0.7, 1),
    };
    vector<size_t> indices = cover_interval(intervals, -0.5f, 1.0f);
    assert(indices.size() == 1);
    assert(indices[0] == 1);

    // 0 1
    // 3
    // 0 0.25
    // 0.25 0.75
    // 0.75 0.999
    // 0 1
    // 3
    // 0 0.25
    // 0.25 0.75
    // 0.75 1
    // 1 1
    // 1
    // 1 1
}
