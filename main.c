#include <stdio.h>
#include <math.h> // Required for log() and exp()

void exponentialFit(float x[], float y[], int n) {
    float sum_x = 0, sum_Y = 0, sum_xY = 0, sum_x2 = 0;

    for (int i = 0; i < n; i++) {
        float Y = log(y[i]); // Transform y to ln(y)
        sum_x += x[i];
        sum_Y += Y;
        sum_xY += x[i] * Y;
        sum_x2 += x[i] * x[i];
    }

    // Solve for b and A (where A = ln(a))
    float b = (n * sum_xY - sum_x * sum_Y) / (n * sum_x2 - sum_x * sum_x);
    float A = (sum_Y - b * sum_x) / n;
    
    float a = exp(A); // Convert A back to a

    printf("Exponential Fit Equation: y = %.4f * e^(%.4fx)\n", a, b);
}

int main() {
    // Example dataset (y values must be > 0 for log)
    float x[] = {1, 2, 3, 4, 5};
    float y[] = {2.7, 7.4, 20.1, 54.6, 148.4};
    int n = sizeof(x) / sizeof(x[0]);

    exponentialFit(x, y, n);
    return 0;
}