#include <stdio.h>
#include <math.h>

// g(x) = cube root of (2x - 5)
double g(double x) {
    return cbrt(2*x - 5);
}

int main() {
    double x0 = -2.5;   // initial guess (negative root region)
    double x1;
    double tolerance = 0.01;
    int i = 0, maxIterations = 100;

    printf("Iteration\t x0\t\t x1\n");

    while (i < maxIterations) {

        x1 = g(x0);

        printf("%d\t\t %.5lf\t %.5lf\n", i + 1, x0, x1);

        if (fabs(x1 - x0) < tolerance) {
            break;
        }

        x0 = x1;
        i++;
    }

    printf("\nApproximate Root = %.5lf\n", x1);

    return 0;
}