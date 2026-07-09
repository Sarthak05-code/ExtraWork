#include <stdio.h>

// Function to calculate u term
float backward_u_cal(float u, int n) {
    float temp = u;
    for (int i = 1; i < n; i++)
        temp = temp * (u + i);
    return temp;
}

// Function to calculate factorial
int fact(int n) {
    int f = 1;
    for (int i = 2; i <= n; i++)
        f *= i;
    return f;
}

void newton_backward(float x[], float y[][10], int n, float value) {
    // Generating Backward Difference Table
    for (int i = 1; i < n; i++) {
        for (int j = n - 1; j >= i; j--) {
            y[j][i] = y[j][i - 1] - y[j - 1][i - 1];
        }
    }

    // Initializing u and sum
    float sum = y[n - 1][0];
    float u = (value - x[n - 1]) / (x[1] - x[0]);
    
    for (int i = 1; i < n; i++) {
        sum = sum + (backward_u_cal(u, i) * y[n - 1][i]) / fact(i);
    }

    printf("\nValue at %f using Newton Backward Interpolation is: %.4f\n", value, sum);
}

int main() {
    int n = 4;
    float x[] = {10, 20, 30, 40};
    
    float y[10][10] = {0};
    y[0][0] = 0.1736;
    y[1][0] = 0.3420;
    y[2][0] = 0.5000;
    y[3][0] = 0.6428;

    float value = 38; // Interpolation point
    newton_backward(x, y, n, value);
    
    return 0;
}