Stack based 2D esoteric programming language where a frog hops across a lilypad grid, croaking out values.

Read [LANGUAGE_SPEC.md](./LANGUAGE_SPEC.md) to see language specifications.

| Tile | Action |
| :--- | :--- |
| '.' | continues, no action |
| '@' | frog turns 90 degrees on the lilypad |
| ':' | frog duplicates last value in its fly stack |
| '0-9' | frog eats that many flies and pushes the value into its stack |
| 's' | frog forgets the last two flies that it ate, and switches their values |
| 'x' | frog forgets last value in stack |
| '+' | frog compounds the last two values in stack |
| '*' | frog multiplies last two values in stack |
| 'p' | frog belches out the last fly |
| 'a' | frog belches out the last as its ASCII equivalent |
| '#' | frog is done eating and ends the run |
