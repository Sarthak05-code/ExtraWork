#include <stdio.h>

#define N 3 // number of equations (change as needed)

void printMatrix(float a[N][N + 1]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= N; j++)
            printf("%8.3f ", a[i][j]);
        printf("\n");
    }
}

void swapRows(float a[N][N + 1], int r1, int r2) {
    for (int j = 0; j <= N; j++) {
        float temp = a[r1][j];
        a[r1][j] = a[r2][j];
        a[r2][j] = temp;
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

    // Forward Elimination with Partial Pivoting
    for (int k = 0; k < N - 1; k++) {

        // Find the row with the largest absolute value in column k (at or below row k)
        int maxRow = k;
        float maxVal = a[k][k];
        if (maxVal < 0) maxVal = -maxVal;

        for (int i = k + 1; i < N; i++) {
            float val = a[i][k];
            if (val < 0) val = -val;
            if (val > maxVal) {
                maxVal = val;
                maxRow = i;
            }
        }

        // Swap rows if a bigger pivot was found below
        if (maxRow != k) {
            swapRows(a, k, maxRow);
        }

        if (a[k][k] == 0) {
            printf("Mathematical Error: matrix is singular!\n");
            return 1;
        }

        // Eliminate entries below the pivot
        for (int i = k + 1; i < N; i++) {
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