import random as ran
import time as t


def return_number():

    return ran.randint(1, 10)


def main():
    array = [i for i in range(10) if i % 2 == 0]
    print("The even array values are : ")
    print(array)
    t.sleep(2)
    for i, arr in enumerate(array):
        array[i] = arr + return_number()

    array_2 = [i for i in range(10) if i % 2 != 0 and i * i == i * i]
    print("The new array value are:")
    print(array)
    t.sleep(2)
    print("The value of array 2 are: ")
    print(array_2)
    t.sleep(2)
    for value1, value2 in zip(array, array_2):
        if value1 == value2:
            print(f"\nThis value {value1} {value2} match")
        else:
            print(f"The values are {value1}, {value2}")


if __name__ == "__main__":
    main()
