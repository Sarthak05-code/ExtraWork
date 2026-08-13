import random as ran


def return_number():

    return ran.randint(1, 10)


def main():
    array = [i for i in range(10) if i % 2 == 0]
    print(array)
    for i, arr in enumerate(array):
        if arr % 2 == 0:
            array[i] = arr + return_number()

    array_2 = [i for i in range(10) if i % 2 != 0 and i * i == i * i]

    print(array)
    print(array_2)

    for i, answer in enumerate(zip(array, array_2)):
        if array[i] == array_2[i]:
            print(f"\nThis value {array[i]} {array_2[i]} match")
        else:
            print(f"The value are {answer}")


if __name__ == "__main__":
    main()
