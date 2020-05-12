# Example of 2 programs using a named pipe in the same directory

Run 'named-pipe-reader' in one terminal and run 'named-pipe-writer' in the other terminal but make sure they are both in the same directory before running.

Then write will pass data to read and exit when finished.

## Compile the writer program

```bash
clang -Wall -Werror named-pipe-reader.c -o named-pipe-reader
```

## Compile the reader program

```bash
clang -Wall -Werror named-pipe-writer.c -o named-pipe-writer
```
