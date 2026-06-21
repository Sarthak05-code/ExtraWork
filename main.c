/*
 * Rotating ASCII Cube
 * ---------------------------------------------------------
 * A full 6-face rotating cube rendered with ASCII characters,
 * using a z-buffer for hidden surface removal.
 *
 * Cross-platform: works on Windows (MSYS2/MinGW, MSVC) and
 * Linux/macOS without needing usleep() or POSIX-only headers.
 *
 * Compile (MSYS2 / MinGW / Linux / macOS):
 *   gcc rotating_cube.c -o cube.exe -lm
 *   ./cube.exe
 *
 * If you're running this in plain Windows cmd.exe and the
 * screen doesn't clear properly, run it from Windows Terminal,
 * or MSYS2's terminal instead -- both support ANSI escape codes.
 * ---------------------------------------------------------
 */

#include <stdio.h>
#include <math.h>
#include <string.h>


#ifdef _WIN32
    #include <windows.h>
    #define SLEEP_MS(ms) Sleep(ms)
#else
    #include <unistd.h>
    #define SLEEP_MS(ms) usleep((ms) * 1000)
#endif

/* ---------- configuration ---------- */
#define WIDTH        120
#define HEIGHT       40
#define CUBE_WIDTH   12.0f
#define STEP         0.6f      /* surface sampling resolution   */
#define DISTANCE     60.0f     /* distance of cube from viewer  */
#define K1           40.0f     /* projection scale factor       */
#define ROT_SPEED    0.03f     /* radians per frame              */
#define FRAME_DELAY  16        /* ms between frames (~60 fps)   */

/* ---------- globals ---------- */
static float angleA = 0, angleB = 0, angleC = 0;
static float zBuffer[WIDTH * HEIGHT];
static char  screen[WIDTH * HEIGHT];

/* ---------- 3D rotation ---------- */
static void rotate(float x, float y, float z, float *ox, float *oy, float *oz) {
    /* rotate around X */
    float y1 = y * cosf(angleA) - z * sinf(angleA);
    float z1 = y * sinf(angleA) + z * cosf(angleA);

    /* rotate around Y */
    float x2 = x * cosf(angleB) + z1 * sinf(angleB);
    float z2 = -x * sinf(angleB) + z1 * cosf(angleB);

    /* rotate around Z */
    float x3 = x2 * cosf(angleC) - y1 * sinf(angleC);
    float y3 = x2 * sinf(angleC) + y1 * cosf(angleC);

    *ox = x3;
    *oy = y3;
    *oz = z2;
}

/* ---------- plot one surface point ---------- */
static void plot(float x, float y, float z, char ch) {
    float rx, ry, rz;
    rotate(x, y, z, &rx, &ry, &rz);
    rz += DISTANCE;

    float ooz = 1.0f / rz;
    int xp = (int)(WIDTH / 2 + K1 * ooz * rx * 2);
    int yp = (int)(HEIGHT / 2 + K1 * ooz * ry);

    int idx = xp + yp * WIDTH;
    if (idx >= 0 && idx < WIDTH * HEIGHT) {
        if (ooz > zBuffer[idx]) {
            zBuffer[idx] = ooz;
            screen[idx] = ch;
        }
    }
}

/* ---------- render one full cube face ---------- */
static void renderFace(int axis, float fixedValue, char ch) {
    for (float i = -CUBE_WIDTH; i < CUBE_WIDTH; i += STEP) {
        for (float j = -CUBE_WIDTH; j < CUBE_WIDTH; j += STEP) {
            switch (axis) {
                case 0: plot(fixedValue, i, j, ch); break; /* +-X face */
                case 1: plot(i, fixedValue, j, ch); break; /* +-Y face */
                case 2: plot(i, j, fixedValue, ch); break; /* +-Z face */
            }
        }
    }
}

int main(void) {
#ifdef _WIN32
    /* enable ANSI escape code support on modern Windows terminals */
    HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);
    DWORD mode = 0;
    GetConsoleMode(hOut, &mode);
    SetConsoleMode(hOut, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
#endif

    while (1) {
        memset(screen, ' ', sizeof(screen));
        memset(zBuffer, 0, sizeof(zBuffer));

        renderFace(0,  CUBE_WIDTH, '@');  /* +X */
        renderFace(0, -CUBE_WIDTH, '#');  /* -X */
        renderFace(1,  CUBE_WIDTH, '$');  /* +Y */
        renderFace(1, -CUBE_WIDTH, '~');  /* -Y */
        renderFace(2,  CUBE_WIDTH, ';');  /* +Z */
        renderFace(2, -CUBE_WIDTH, '+');  /* -Z */

        /* clear screen + move cursor home, then draw frame */
        printf("\x1b[H\x1b[2J");
        for (int row = 0; row < HEIGHT; row++) {
            fwrite(&screen[row * WIDTH], 1, WIDTH, stdout);
            putchar('\n');
        }
        fflush(stdout);

        angleA += ROT_SPEED;
        angleB += ROT_SPEED * 0.7f;
        angleC += ROT_SPEED * 0.5f;

        SLEEP_MS(FRAME_DELAY);
    }

    return 0;
}