#include <stdio.h>

float lagrange_interpolation(float x[], float y[], int n, float value) {
    float result = 0.0; 

    for (int i = 0; i < n; i++) {
        float term = y[i];
        for (int j = 0; j < n; j++) {
            if (j != i) {
                term = term * (value - x[j]) / (x[i] - x[j]);
            }
        }
        result += term;
    }

    return result;
}

int main() {
    int n = 4;
    // Unequally spaced points work fine here
    float x[] = {5, 6, 9, 11};
    float y[] = {12, 13, 14, 16};
    
    float value = 10; // Interpolation point
    
    float res = lagrange_interpolation(x, y, n, value);
    printf("\nValue at %f using Lagrange Interpolation is: %.4f\n", value, res);
    
    return 0;
}