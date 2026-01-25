# Day 10: Monitoring Station

Polar coordinates. BTreeMap and binary_search

## Part 1

Find best location -- with most asteroids visible to place station.
use a line intersection algorithm to see if there are any points in the 
system between a chosen point and all other points. Kind of clunky but
works.

## Part 2

from best location, clockwise zap them, until you zap 200.
Same idea from part 1 worked here, to find visible, then look for the
200th one. Fortunately, the first visibility pass was >200 asteroids.

better idea

from best location, convert all visible to polar coordinates
sort by angle (0 = up), remove all visible (until 200), if
run out, scan for next visible, repeat until zapped 200.

back-patch part 1 to have each point contain a map of 
all other points in relative polar coordinates keyed by 
`theta` and sorted by `r`.

Then visible ones are the number of items in the theta map of the
selected asteroid. To zap them, just loop through in theta order
to get the 200th. In our case it would be the first of the mapped lists
as there are more than 200 unique thetas from the best location

this worked well. a bit to understand more of how Rust does things
but overall looks like less code and cleaner. The algorithm feels better