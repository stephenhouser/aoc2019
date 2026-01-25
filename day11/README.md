# Day 11: Space Police

Intcode variation

## Part 1

More or less, pump the Intcode computer...

Robot starts facing up at (x,y). All panels are black (0).

Input
---

- 0 if the robot is over a black panel
- 1 if the robot is over a white panel

Output
---

1. color to paint the panel the robot is over: 
    - 0 means to paint the panel black
    - 1 means to paint the panel white.
2. it will output a value indicating the direction the robot should turn
    - 0 means it should turn left 90 degrees
    - 1 means it should turn right 90 degrees.

Move forward 1 panel

Result
---

Number of panels painted at least once

## Part 2
