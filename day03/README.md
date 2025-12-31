# Day 3: Crossed Wires

## Part 1

Most work seemed to be in creating a `Point` class that had an `add` method
and was orderable (`Ord`) to be used with `min()`. Created a `HashSet` of points
for each wire and then found the intersections by intersecting the sets. Then
`min()` could do it's work using Manhattan distance.

Had one problem early on; wires cross themselves. Need to only count intersections
of wires. where the wires cross not where the wires cross themselves

## Part 2

Converted to `HashMap` for each wire and used the value as the number of steps
from starting point. Then, same as part 1, find intersections. Evaluate
each intersection for "close" by comparing the sum of the steps to get there.
Choose the smallest, again with `min()`.

Don't you hate it when you have the right answer/solution but added wrong elsewhere and thought your algorithm was wrong. Good to have saved the code! Had it close to right and then
thought it had to be the min steps to get to the closest point. This sent me off track.
Then I went back and re-read the directions. Had it right the first time.