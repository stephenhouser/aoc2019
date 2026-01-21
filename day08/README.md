# Day 8: Space Image Format

Map, reduce, filter

## Part 1

Could be I'm just getting the hang of map, reduce, and filter type operations.
This problem did not even really need to parse the input into the 2D map as
described. Break into chunks (one for each layer) then map/transform into
a vector of how many of each digit (digit counts). Use that to find the one
with the least 0s and return the 1s * 2s.

## Part 2

Another map, reduce, filter. Break into layers, then apply each layer
over an accumulated layer, replacing pixels that are transparent.

This was also one of those where you have to print a picture and interpret
what it shows. Text in this case. Not easily automated testing with my setup.