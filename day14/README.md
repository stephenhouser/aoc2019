# Day 14: Care Package

A recursive problem perhaps, did it iteratively thinking part 2 would
kill recursion.

## Part 1

This one took a long time, lots of breaks in between, just
did not have the energy and focus.

Parsing was done early, changed data representation a few times to
suit changing view of problem.

Wanted iterative solution.
Rules for production kep in HashMap for easy access.
Needed to keep track of surplus elements (chemicals) created,
only realized this after getting initial solution working.

Put what you want in a queue, then process the queue, working
backwards through the rules, adding to the queue the things
that were needed to produce the element (in quantity).

When you only have ore to be produced, you are done.

## Part 2

Use solution for part 1 and binary search for solution.
use lower bound as 1 trillion / the cost to produce one FUEL.
use upper bound as 1 trillion / (the cost to produce / 2)
The upper bound seems overly generous.
