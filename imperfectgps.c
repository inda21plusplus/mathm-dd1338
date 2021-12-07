#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>
#include <math.h>

double lerp(double p, double in_from, double in_to, double out_from, double out_to) {
    return (p - in_from) / (in_to - in_from) * (out_to - out_from) + out_from;
}

double dist(int x1, int y1, int x2, int y2) {
    int64_t dx = x1-x2;
    int64_t dy = y1-y2;
    return sqrt(dx*dx + dy*dy);
}

int main(void) {
    int n, dt;
    scanf("%d%d", &n, &dt);
    int xs[100], ys[100], ts[100];

    for (int i = 0; i < n; ++i)
        scanf("%d%d%d", &xs[i], &ys[i], &ts[i]);

    int i = 1;
    int p_x = xs[0], p_y = ys[0];
    double gps_dist = 0.0;
    for (int t = 0; i < n; t += dt) {
        while (i < n && t >= ts[i]) ++i;
        if (i >= n) break;
        double x = lerp(t, ts[i-1], ts[i], xs[i-1], xs[i]);
        double y = lerp(t, ts[i-1], ts[i], ys[i-1], ys[i]);
        gps_dist += dist(p_x, p_y, x, y);
        p_x = x;
        p_y = y;
    }
    gps_dist += dist(p_x, p_y, xs[n-1], ys[n-1]);

    double true_dist = 0.0;
    for (int i = 1; i < n; ++i)
        true_dist += dist(xs[i-1], ys[i-1], xs[i], ys[i]);

    printf("%.14f\n", (true_dist - gps_dist) * 100.0 / true_dist);
}
