## Keywords

| Symbol | Description                                                                  |
| ------ | ---------------------------------------------------------------------------- |
| `.`    | Moves Baer to the next sequence in the collatz conjecture for the current x. |
| `+`    | Increments the current column and performs `.`                               |
| `-`    | Decrements the current column and performs `.`                               |
| `*`    | Moves 3x + 1 if the column's value is 0, otherwise performs `.`              |
| `<`    | Reads standard input and overwrites the column value with it                 |
| `>`    | Prints column value to standard output                                       |
| `^`    | Moves up one row and performs `.`                                            |
| `v`    | Down down one row and performs `.`                                           |
| `#`    | Comment; Ignores the rest of the line                                        |

### Example 1: 1char.bs

```
<v
>v
```

1. `<` reads from stdin and stores the value in column 1, it moves to (4, 1)
2. Nothing at (4, 1), moves to (2, 1) -> (1, 2)
3. `>` prints the value of column 1 to stdout, then moves to (4, 2) -> (2, 2) -> (1, 3)
4. (1, 3) is out of bounds, so it exits after 6 steps

### Example 2: 5char.bs

```
<*.<..<...<.v...<
.v...............
>*.>..>...>.v...>
```

1. `<` reads from stdin, stores in column 1, moves to (4, 1)
2. `<` reads from stdin, stores in column 3, moves to (2, 1) -> (7, 1)
3. `<` reads from stdin
   ...
4. `v` moves to (40, 2) -> ... -> (2, 2)
5. `v` moves to (1, 3)
6. The third row is identical to the first one, except that it prints to stdout instead of reading from stdin. This will print out all five inputted integers (or characters with the --ascii flag)
