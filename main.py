import random as ran


def return_number():

    return ran.randint(1, 10)


def main():
    array = [i for i in range(10) if i % 2 == 0]
    print(array)
    for i, arr in enumerate(array):
        if arr % 2 == 0:
            array[i] = arr + return_number()
    print("Hello, world")
    print(array)


if __name__ == "__main__":
    main()
