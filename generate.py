import sys
import random

def generate_data(num_rows, max_num):
    with open('dane.txt', 'w') as file:
        file.write(f"{num_rows}\n")
        for i in range(1, num_rows + 1):
            x = random.uniform(0, max_num)
            y = random.uniform(0, max_num)
            file.write(f"{i} {x} {y}\n")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python generate.py <num_rows> <max_num>")
        sys.exit(1)

    num_rows = int(sys.argv[1])
    max_num = float(sys.argv[2])

    generate_data(num_rows, max_num)
    print(f"Generated {num_rows} rows of data with max number {max_num} in dane.txt")