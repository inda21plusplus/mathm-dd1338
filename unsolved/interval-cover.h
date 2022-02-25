#ifndef INTERVAL_COVER_H
#define INTERVAL_COVER_H

#include <vector>
#include <algorithm>
#include <limits>

#include <iostream>

template<typename T>
std::vector<size_t> cover_interval(std::vector<std::pair<T, T>>& avail, T begin, T end) {
    if (begin == end) {
        for (size_t i = 0; i < avail.size(); ++i) {
            if (avail[i].first <= begin && avail[i].second >= end) {
                return { i };
            }
        }
        return {};
    }

    std::vector<size_t> chosen;

    std::sort(avail.begin(), avail.end());

    T point = begin;
    size_t i = 0;
    while (point < end) {
        T next = std::numeric_limits<T>::min();
        size_t best = -1;
        while (i < avail.size() && avail[i].first <= point) {
            if (avail[i].second > next) {
                next = avail[i].second;
                best = i;
            }
            ++i;
        }
        if (best == -1ul) return {};
        chosen.push_back(best);
        point = next;
    }

    return chosen;
}

#endif // INTERVAL_COVER_H
