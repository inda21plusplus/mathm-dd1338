// https://open.kattis.com/problems/gamerank

#include <stdio.h>
#include <stdbool.h>

int max_stars[] = {
    [1]  = 5,
    [2]  = 5,
    [3]  = 5,
    [4]  = 5,
    [5]  = 5,
    [6]  = 5,
    [7]  = 5,
    [8]  = 5,
    [9]  = 5,
    [10] = 5,

    [11] = 4,
    [12] = 4,
    [13] = 4,
    [14] = 4,
    [15] = 4,

    [16] = 3,
    [17] = 3,
    [18] = 3,
    [19] = 3,
    [20] = 3,

    [21] = 2,
    [22] = 2,
    [23] = 2,
    [24] = 2,
    [25] = 2,
};

int main(void) {
    int rank = 25;
    int stars = 0;
    int streak = 0;
    char buffer[10002]; fgets(buffer, 10002, stdin);
    for (int i = 0; buffer[i] != 0; ++i) {
        if (buffer[i] == 'W') {
            streak++;
            if (streak >= 3 && rank >= 6) stars++;
            stars++;
            if (stars > max_stars[rank]) {
                stars -= max_stars[rank];
                rank--;
            }
        }
        if (buffer[i] == 'L') {
            streak = 0;
            if (rank <= 20) {
                stars--;

                if (stars < 0) {
                    if (rank == 20) {
                        stars = 0;
                    } else {
                        rank++;
                        stars = max_stars[rank] - 1;
                    }
                }
            }
        }
        if (rank == 0) {
            printf("Legend\n");
            return 0;
        }
    }
    printf("%d\n", rank);
}
