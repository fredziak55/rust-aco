import matplotlib.pyplot as plt

# Read the file
with open('data/dane.txt', 'r') as file:
    lines = file.readlines()

# First line is the number of points
num_points = int(lines[0].strip())

# Initialize lists to hold the coordinates
x_coords = []
y_coords = []
point_numbers = []

# Read the points
for line in lines[1:num_points+1]:
    parts = line.strip().split()
    point_number = int(parts[0])
    x = float(parts[1])
    y = float(parts[2])
    x_coords.append(x)
    y_coords.append(y)
    point_numbers.append(point_number)

# Example array of connections (indices of points to connect)
connections = []

# Set the figure size
plt.figure(figsize=(12, 10))

# Plot the points
plt.scatter(x_coords, y_coords)

# Annotate the points with their numbers
for i, point_number in enumerate(point_numbers):
    plt.annotate(point_number, (x_coords[i], y_coords[i]))

# Connect the points based on the connections array
for i in range(len(connections) - 1):
    start_index = connections[i]
    end_index = connections[i + 1]
    plt.plot([x_coords[start_index], x_coords[end_index]], [y_coords[start_index], y_coords[end_index]], 'k-')

# Add grid
plt.grid(True)

plt.xlabel('X')
plt.ylabel('Y')

# Save the plot to a file
plt.savefig('instance.png')