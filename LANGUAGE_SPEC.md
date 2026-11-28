# Croakhop Language Specification
Ver 0.0.1

Every hop, froggo goes through this process:
1. move frog
2. check destination lilypad for instruction
3. execute instruction
4. continue

 - Froggo starts on 0, 0 (top left) unless otherwise specified.
 - Hitting a wall reverses direction: if destination is outside pond, frog reverses relavent direction axis. Then frog goes to the adjacent valid cell.
 - Hitting a corner reverses both direction axis.
 - When stack is empty frog defaults values to zero.

 | Tile | Action |
| :--- | :--- |
| '.' | continues, no action |
| '@' | frog turns 90 degrees on the lilypad |
| '^ v' | frog points in that direction, preserving its velocity. dx is set to 0. If dy = 0, dy will have a speed of 1. |
| '< >' | frog points in that direction, preserving its velocity. dy is set to 0. If dx = 0, dx will have a speed of 1. |
| 'c' | multiplies velocity of frog by a factor of 2 (dx * 2, dy * 2) |
| ':' | frog duplicates last value in its fly stack |
| '0-9' | frog eats that many flies and pushes the value into its stack |
| 's' | frog forgets the last two flies that it ate, and switches their values |
| 'x' | frog forgets last value in stack |
| '+' | frog compounds the last two values in stack. Treats missing operands as 0. |
| '-' | frog subtracts the last two values in stack, the second top value minus the top value |
| '*' | frog multiplies last two values in stack |
| '/' | frog divides the last two values in stack, the second top value divided by the top value and pushes as a truncated integer. If divisor is zero will preserve dividend. |
| '%' | frog returns the remainder of the last two values in stack, the second top value divided by the top value. If divisor is zero will preserve dividend. |
| 'p' | frog belches (pops) out the last fly |
| 'a' | frog belches (pops) out the last as its ASCII equivalent |
| '#' | frog is done eating and ends the run |
