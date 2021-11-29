#ifndef INTERVAL_COVER_H
#define INTERVAL_COVER_H

#include <vector>
#include <algorithm>
#include <limits>

template<typename T>
std::vector<size_t> cover_interval(const std::vector<std::pair<T, T>>& avail, T begin, T end) {
    std::vector<size_t> chosen;

    std::sort(avail.begin(), avail.end());

    T point = begin;
    while (point < end) {
        size_t i = 0;
        T next = std::numeric_limits<T>::min();
        size_t best;
        while (avail[i].first <= point) {
            if (avail[i].second > next) {
                next = avail[i].second;
                best = i;
            }
        }
        chosen.push_back(best);
        point = next;
    }

    return chosen;
}

#endif // INTERVAL_COVER_H
