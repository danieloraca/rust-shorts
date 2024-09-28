# Simple exercises

## Exercise 1 - tuple check
In the checker function, the tuple (1, 2) is matched against the pattern (x, _) | (_, x) with a guard if check(x):

The pattern (x, _) | (_, x) attempts to bind both elements of the tuple to x:
First, it tries to match (x, _), binding x = 1.
It then calls check(1), which prints 1 but returns false, so the first match fails.
The second part of the pattern (_, x) binds x = 2.
It then calls check(2), printing 2, but again check(2) returns false.
Since both checks fail, the match proceeds to the default case (_), printing "4".

Thus, the output sequence is: 1 2 4.
