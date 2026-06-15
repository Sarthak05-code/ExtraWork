#include <stdio.h>

static const char SYS_CLEAR[] = "\033[H\033[2J";

typedef struct
{
    char *name;
    double hp;
    char *buffer;
    double *buff;
    double *damage;

} Character_1;

typedef struct
{
    char *name;
    double hp;
    char *buffer;
    double *heal;
    double *damage;

} Character_2;

typedef struct
{
    char *name;
    double hp;
    char *buffer;
    double *buff;
    double *damage;

} Character_3;

typedef struct
{
    char *name;
    double hp;
    char *buffer;
    double *self_buff;
    double *damage;

} Character_4;

int main()
{
    printf("%s", SYS_CLEAR);

    double array[] = {4250.0, 6230.0, 3230.0, 2930.0};

    double buff_val = 20.0;
    double damage_val = 0.0;
    double heal_val = 50.0;
    double damage_val2 = 75.0;
    double buff_val3 = 15.0;
    double damage_val3 = 30.0;
    double self_buff_val4 = 10.0;
    double damage_val4 = 40.0;
    Character_1 s1 = {"Cyrene", array[0], "True Damage", &buff_val, &damage_val};
    Character_2 s2 = {"Blade", array[1], "Vulnerability", &heal_val, &damage_val2};
    Character_3 s3 = {"Ashveil", array[2], "Follow-up attack buff", &buff_val3, &damage_val3};
    Character_4 s4 = {"Acheron", array[3], "Def-shred", &self_buff_val4, &damage_val4};

    printf("The name of the first character is : %s\n", s1.name);
    printf("The name of the second character is : %s\n", s2.name);
    printf("The name of the third character is : %s\n", s3.name);
    printf("The name of the fourth character is : %s\n", s4.name);

    double highest = s1.hp;

    for (int i = 0; i < 4; ++i)
    {
        if (s2.hp > highest)
        {
            highest = s2.hp;
        }
        else if (s3.hp > highest)
        {
            highest = s3.hp;
        }
        else if (s4.hp > highest)
        {
            highest = s4.hp;
        }
        else
        {
            // nothing as the s1.hp will be called the highest as default
        }
    }

    printf("The highest hp is: %.2lf ", highest);

    return 0;
}