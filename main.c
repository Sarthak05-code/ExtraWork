#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

#define N 5
#define MAX_CYCLES 5
#define LEFT(id)  ((id + N - 1) % N)
#define RIGHT(id) ((id + 1) % N)

typedef enum { THINKING, HUNGRY, EATING } state_t;

state_t state[N];
int meals_eaten[N] = {0};

pthread_mutex_t mutex;          // protects shared state[] array
pthread_cond_t cond[N];         // one condition variable per philosopher
pthread_t philosophers[N];

void print_state()
{
    printf("\033[H\033[2J"); // clear screen for a live "dashboard" view
    printf("Philosopher states:\n");
    for (int i = 0; i < N; i++)
    {
        const char *s = state[i] == THINKING ? "THINKING" :
                         state[i] == HUNGRY   ? "HUNGRY"   : "EATING";
        printf("  P%d: %-8s (meals eaten: %d)\n", i, s, meals_eaten[i]);
    }
    printf("\n");
}

// Check if philosopher i can start eating:
// it must be HUNGRY and neither neighbor can currently be EATING.
void test(int i)
{
    if (state[i] == HUNGRY &&
        state[LEFT(i)] != EATING &&
        state[RIGHT(i)] != EATING)
    {
        state[i] = EATING;
        print_state();
        pthread_cond_signal(&cond[i]); // wake up philosopher i if it was waiting
    }
}

void think(int id)
{
    sleep(rand() % 2 + 1);
}

void eat(int id)
{
    sleep(rand() % 2 + 1);
}

void take_forks(int id)
{
    pthread_mutex_lock(&mutex);
    state[id] = HUNGRY;
    print_state();
    test(id); // see if we can eat immediately

    // If we couldn't transition to EATING, wait until signaled by a neighbor
    while (state[id] != EATING)
    {
        pthread_cond_wait(&cond[id], &mutex);
    }
    pthread_mutex_unlock(&mutex);
}

void put_forks(int id)
{
    pthread_mutex_lock(&mutex);
    state[id] = THINKING;
    meals_eaten[id]++;
    print_state();

    // Re-check neighbors: they may now be able to eat
    test(LEFT(id));
    test(RIGHT(id));
    pthread_mutex_unlock(&mutex);
}

void *philosopher_routine(void *args)
{
    int id = *(int *)args;

    for (int cycle = 0; cycle < MAX_CYCLES; cycle++)
    {
        think(id);
        take_forks(id);
        eat(id);
        put_forks(id);
    }
    return NULL;
}

int main()
{
    int ids[N];

    pthread_mutex_init(&mutex, NULL);
    for (int i = 0; i < N; i++)
    {
        state[i] = THINKING;
        pthread_cond_init(&cond[i], NULL);
    }

    for (int i = 0; i < N; i++)
    {
        ids[i] = i;
        pthread_create(&philosophers[i], NULL, philosopher_routine, &ids[i]);
    }

    for (int i = 0; i < N; i++)
    {
        pthread_join(philosophers[i], NULL);
    }

    printf("\nFinal meal counts:\n");
    for (int i = 0; i < N; i++)
    {
        printf("  Philosopher %d ate %d times\n", i, meals_eaten[i]);
    }

    pthread_mutex_destroy(&mutex);
    for (int i = 0; i < N; i++)
    {
        pthread_cond_destroy(&cond[i]);
    }

    return 0;
}