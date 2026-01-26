# Day 12: The N-Body Problem

Points, Vectors, Iteration

## Part 1

simulate movement of moons based on gravity changing the velocity and
the velocity changing positions.

Mostly detailed work to get the sequence correct.

## Part 2

Find where the system begins to loop. Where matches a previous state.

Flibbergibbets, I think it's a chinese remainder therom problem or similar.

look for loops in each independent axis; x, y, z

- cache x axis and a velocity across all moons
- cache y axis and a velocity across all moons
- cache z axis and a velocity across all moons

- look for when those repeat for the loop and loop length

- least common multiple of the loop lengths

could stop checking axis where loop was already found to make faster?