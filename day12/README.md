# Day 12: The N-Body Problem

Points, Vectors, Iteration

## Part 1

simulate movement of moons based on gravity changing the velocity and
the velocity changing positions.

Mostly detailed work to get the sequence correct.

## Part 2

Find where the system begins to loop. Where matches a previous state.

Flibbergibbets, I think it's a chinese remainder therom problem or similar.

- for a moon; will repeat when all moon positions are same and my vel repeats
    next velocity is based only on relative positions.

- find a moon's cycle

- for ecah moon cache positions+my_vel

- simulate until each moon has repeated

- multiply those together for least common 