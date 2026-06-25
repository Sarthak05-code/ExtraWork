#include <math.h>
#include <stdio.h>
#define f1(x, y, z) (17 - y + 2 * z) / 20
#define f2(x, y, z) (-18 - 3 * x + z) / 20
#define f3(x, y, z) (25 - 2 * x + 3 * y) / 20
#define e 0.00001
int main() {
  double x0 = 0, y0 = 0, z0 = 0, x1, y1, z1, e1, e2, e3;
  int count = 1;
  printf("Gauss - seidal version\n");
  printf("\n Count\t\tx\t\ty\t\tz\n");
  do {
    x1 = f1(x0, y0, z0);
    e1 = fabs(x0 - x1);
    y1 = f2(x1, y0, z0);
    e2 = fabs(y0 - y1);
    z1 = f3(x1, y1, z0);
    e3 = fabs(z0 - z1);
    printf("%d\t\t%.4lf\t\t%.4lf\t\t%.4lf\n", count, x1, y1, z1);
    x0 = x1; // ← fix
    y0 = y1; // ← fix
    z0 = z1; // ← fix
    count++;
  } while (e1 > e || e2 > e || e3 > e);
  printf("Solution x : %.3lf , y : %.3lf , z : %.3lf", x1, y1, z1);
  return 0;
}
