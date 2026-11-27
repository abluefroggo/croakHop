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
| ':' | frog duplicates last value in its fly stack |
| '0-9' | frog eats that many flies and pushes the value into its stack |
| 's' | frog forgets the last two flies that it ate, and switches their values |
| 'x' | frog forgets last value in stack |
| '+' | frog compounds the last two values in stack |
| '-' | frog subtracts the last two values in stack, the second top value minus the top value |
| '*' | frog multiplies last two values in stack |
| '/' | frog divides the last two values in stack, the second top value divided by the top value as a truncated integer |
| '%' | frog returns the remainder of the last two values in stack, the second top value divided by the top value |
| 'p' | frog belches (pops) out the last fly |
| 'a' | frog belches (pops) out the last as its ASCII equivalent |
| '#' | frog is done eating and ends the run |
