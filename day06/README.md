# Day 6: Universal Orbit Map

HashMap, iteration

## Part 1

Make input into hashmap, so each planet's value is the planet it orbits.
Create a vector of the path from the planet to the center (COM). Do this
for all planets and sum the length of the vectors (the distance).

Could speed this up with a cache so we don't recompute distances for inner
planets repeatedly. Make `HashMap<String, (String, distance)>`

## Part 2

Find the paths for the `YOU` and `SAN`, then look for the first common
planet in both of the paths. Find the distance from each of the starting
points to this common planet.

