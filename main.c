#include <stdio.h>
//Forward
// Function to calculate u term
float forward_u_cal(float u, int n) {
    float temp = u;
    for (int i = 1; i < n; i++)
        temp = temp * (u - i);
    return temp;
}

// Function to calculate factorial
int fact(int n) {
    int f = 1;
    for (int i = 2; i <= n; i++)
        f *= i;
    return f;
}

void newton_forward(float x[], float y[][10], int n, float value) {
    // Generating Forward Difference Table
    for (int i = 1; i < n; i++) {
        for (int j = 0; j < n - i; j++) {
            y[j][i] = y[j + 1][i - 1] - y[j][i - 1];
        }
    }

    // Initializing u and sum
    float sum = y[0][0];
    float u = (value - x[0]) / (x[1] - x[0]);
    
    for (int i = 1; i < n; i++) {
        sum = sum + (forward_u_cal(u, i) * y[0][i]) / fact(i);
    }

    printf("\nValue at %f using Newton Forward Interpolation is: %.4f\n", value, sum);
}

int main() {
    int n = 4;
    float x[] = {10, 20, 30, 40};
    
    // y[][0] is used for storing input y-values
    float y[10][10] = {0};
    y[0][0] = 0.1736;
    y[1][0] = 0.3420;
    y[2][0] = 0.5000;
    y[3][0] = 0.6428;

    float value = 15; // Interpolation point
    newton_forward(x, y, n, value);
    
    return 0;
}