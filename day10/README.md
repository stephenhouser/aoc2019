# Day 10: Monitoring Station


## Part 1

Find best location -- with most asteroids visible to place station

## Part 2

from best location, clockwise zap them, until you zap 200.

from best location, convert all visible to polar coordinates
sort by angle (0 = up), remove all visible (until 200), if
run out, scan for next visible, repeat until zapped 200.

could back-patch part 1 to have each point contain a map of 
all other points in relative polar coordinates keyed by 
`theta` and sorted by `r`.

Then visible ones are the number of items in the theta map of the
selected asteroid. To zap them, just loop through in theta order
to get the 200th. In our case it would be the first of the mapped lists
as there are more than 200 unique thetas from the best location