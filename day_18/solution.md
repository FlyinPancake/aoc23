# Advent of Code 2023, Day 18

[Link](https://adventofcode.com/2023/day/18)

## Part 1

We are given instructions to dig a hole in the ground. The instructions include direction to dig and the distance to dig. They also include the colour of the trench to paint, but that is not relevant to this part of the problem.
With the instruction set, we create a loop of trenches. We then are asked to dig out inside the loop. We are asked to find the total dug out area.

This problem seems to be similar to the [Day 10 part 2](https://adventofcode.com/2023/day/10#part2) problem. I could use the same approach here, but I will try to solve this problem using a different approach.

## [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)

Using the shoelace formula, we can find the area of a polygon given the coordinates of the vertices. Using this formula to find the area of the loop, will not work. The area would be too small. The shoelace theorem relies on exact outer coordinates for a polygon, but ours are centre points of the trenches. We could use the shoelace formula to find the area of the polygon formed by the centre points of the trenches, but that would not be the area of the loop.

$$
A = \frac{\left| \sum_{i=1}^{n-1} x_i(y_{i+1}-y_{i-1}) \right| } 2
$$

Where $A$ is the area of the polygon, $n$ is the number of vertices, $x_i$ is the $x$ coordinate of the $i$th vertex and $y_i$ is the $y$ coordinate of the $i$th vertex.

## [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem)

We combine the shoelace formula with Pick's theorem. Pick's theorem states that the area of a polygon with integer coordinates is equal to the number of lattice points inside the polygon plus half the number of lattice points on the boundary minus one. We can use the shoelace formula to find the area of the polygon and Pick's theorem to find the number of lattice points inside the polygon.

$$
A = i + \frac{b}{2} - 1
$$

Where $A$ is the area of the polygon, $i$ is the number of lattice points inside the polygon and $b$ is the number of lattice points on the boundary of the polygon.

## Finding the number of points inside the boundary

We already know $A$ and $b$. We can use Pick's theorem to find $i$.

$$
i = A - \frac{b}{2} + 1
$$
