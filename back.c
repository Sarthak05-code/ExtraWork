#include <stdio.h>

void linearFit(float x[], float y[], int n) {
    float sum_x = 0, sum_y = 0, sum_xy = 0, sum_x2 = 0;

    for (int i = 0; i < n; i++) {
        sum_x += x[i];
        sum_y += y[i];
        sum_xy += x[i] * y[i];
        sum_x2 += x[i] * x[i];
    }

    // Formula for slope (a) and intercept (b)
    float a = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    float b = (sum_y - a * sum_x) / n;

    printf("Linear Fit Equation: y = %.4fx + %.4f\n", a, b);
}

int main() {
    // Example dataset
    float x[] = {1, 2, 3, 4, 5};
    float y[] = {2, 3.5, 5, 6.5, 8};
    int n = sizeof(x) / sizeof(x[0]);

    linearFit(x, y, n);
    return 0;
}