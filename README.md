# H

For the dead week 𝓮𝓷𝓳𝓸𝔂𝓮𝓻. A sophisticated twist on [Fernsicles/h](https://github.com/Fernsicles/h).

## Usage

```
H [eEdD] (-|.*)
```

`e|E` **encodes** capture group one. `d|D` **decodes** capture group one. Specifying `-` reads from stdin
as capture group one, and anything else reads the remaining arguments as capture group one.

## How

For encoding, the program parses your input as a bitstream of half-byte values, converting each to
some variant of the letter H.

For decoding, the program parses your input as a `Vec<char>`, then performs the reverse lookup. If
an invalid `char` is passed, the program will interpret it as a zero byte `0x00`.

Since we can't declare a static hash table in Rust without use of external dependencies, we just use
functions instead.
