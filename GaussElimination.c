#include <stdio.h>

#define N 3 // number of equations (change as needed)

void printMatrix(float a[N][N + 1]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= N; j++)
            printf("%8.3f ", a[i][j]);
        printf("\n");
    }
}

int main() {
    // Augmented matrix [A | b], size N x (N+1)
    float a[N][N + 1] = {
        {2, 1, -1, 8},
        {-3, -1, 2, -11},
        {-2, 1, 2, -3}
    };

    float x[N]; // solution vector

    printf("Initial Augmented Matrix:\n");
    printMatrix(a);

    // Forward Elimination
    for (int k = 0; k < N - 1; k++) {
        for (int i = k + 1; i < N; i++) {
            if (a[k][k] == 0) {
                printf("Mathematical Error: zero pivot encountered!\n");
                return 1;
            }
            float factor = a[i][k] / a[k][k];
            for (int j = k; j <= N; j++) {
                a[i][j] = a[i][j] - factor * a[k][j];
            }
        }
    }

    printf("\nMatrix after Forward Elimination:\n");
    printMatrix(a);

    // Back Substitution
    for (int i = N - 1; i >= 0; i--) {
        x[i] = a[i][N];
        for (int j = i + 1; j < N; j++) {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }

    printf("\nSolution:\n");
    for (int i = 0; i < N; i++)
        printf("x%d = %.4f\n", i + 1, x[i]);

    return 0;
}