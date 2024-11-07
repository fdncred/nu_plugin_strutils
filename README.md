# nu_plugin_strutils

This is a [Nushell](https://nushell.sh/) plugin called "strutils".

## Available Commands

* `str compress` - Compress a string using brotli, flate, or zlib
* `str decompress` - Decompress a string using brotli, flate, or zlib
* `str dedent` - Remove common leading whitespace from text
* `str deunicode` - Replace unicode characters with ASCII counterparts
* `str indent` - Add leading spaces to each line of text
* `str similarity` - Calculate edit distance between strings using various algorithms
* `str wrap` - Wrap text to a specified width

## Installing

```nushell
> cargo install --path .
```

## String utilities for nushell

This plugin implements a some string utilities that are not included in nushell.

### str deunicode
`str deunicode` replaces unicode accented characters with their ASCII counterparts based on the [deunicode crate](https://docs.rs/deunicode/latest/deunicode/).

#### Usage:

```nushell
> plugin add ~/.cargo/bin/nu_plugin_strutils
> plugin use strutils
> 'A…C' | str deunicode
A...C
```

### str similarity
`str similarity` is an older plugin that I thought fit here since this is a common plugin for string utilities.

This plugin uses the [textdistance.rs](https://crates.io/crates/textdistance) crate to calculate edit distance

#### Usage:

#### Single Algorithm Usage

```nushell
❯ "nushell" | str similarity "nutshell" --algorithm levenshtein
1
```

#### All Algorithms Usage

```nushell
❯ "nushell" | str similarity "nutshell" -all
```
```
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │ distance │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │        1 │
│  1 │ cosine                     │     0.94 │
│  2 │ damerau_levenshtein        │        1 │
│  3 │ entropy_ncd                │     0.05 │
│  4 │ hamming                    │        5 │
│  5 │ jaccard                    │     0.88 │
│  6 │ jaro                       │     0.96 │
│  7 │ jaro_winkler               │     0.97 │
│  8 │ levenshtein                │        1 │
│  9 │ longest_common_subsequence │        7 │
│ 10 │ longest_common_substring   │        5 │
│ 11 │ length                     │        1 │
│ 12 │ lig3                       │     0.86 │
│ 13 │ mlipns                     │        0 │
│ 14 │ overlap                    │        1 │
│ 15 │ prefix                     │        2 │
│ 16 │ ratcliff_obershelp         │     0.93 │
│ 17 │ roberts                    │     0.93 │
│ 18 │ sift4_common               │        1 │
│ 19 │ sift4_simple               │        1 │
│ 20 │ smith_waterman             │        6 │
│ 21 │ sorensen_dice              │     0.93 │
│ 22 │ suffix                     │        5 │
│ 23 │ tversky                    │     0.88 │
│ 24 │ yujian_bo                  │     0.12 │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │ distance │
╰────┴────────────────────────────┴──────────╯
```

#### All Algorithms Normalized Usage

The output is normalized between 0 and 1

```nushell
❯ "nushell" | str similarity "nutshell" --all --normalize
```
```
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │ distance │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │     0.12 │
│  1 │ cosine                     │     0.94 │
│  2 │ damerau_levenshtein        │     0.12 │
│  3 │ entropy_ncd                │     0.05 │
│  4 │ hamming                    │     0.62 │
│  5 │ jaccard                    │     0.88 │
│  6 │ jaro                       │     0.96 │
│  7 │ jaro_winkler               │     0.97 │
│  8 │ levenshtein                │     0.12 │
│  9 │ longest_common_subsequence │     0.88 │
│ 10 │ longest_common_substring   │     0.62 │
│ 11 │ length                     │     0.12 │
│ 12 │ lig3                       │     0.86 │
│ 13 │ mlipns                     │        0 │
│ 14 │ overlap                    │        1 │
│ 15 │ prefix                     │     0.25 │
│ 16 │ ratcliff_obershelp         │     0.93 │
│ 17 │ roberts                    │     0.93 │
│ 18 │ sift4_common               │     0.12 │
│ 19 │ sift4_simple               │     0.12 │
│ 20 │ smith_waterman             │     0.75 │
│ 21 │ sorensen_dice              │     0.93 │
│ 22 │ suffix                     │     0.62 │
│ 23 │ tversky                    │     0.88 │
│ 24 │ yujian_bo                  │     0.12 │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │ distance │
╰────┴────────────────────────────┴──────────╯
```

#### List the available algorithms and aliases

```nushell
❯ "nushell" | str similarity "nutshell" --list
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │  alias   │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │ bag      │
│  1 │ cosine                     │ cos      │
│  2 │ damerau_levenshtein        │ dlev     │
│  3 │ entropy_ncd                │ entncd   │
│  4 │ hamming                    │ ham      │
│  5 │ jaccard                    │ jac      │
│  6 │ jaro                       │ jar      │
│  7 │ jaro_winkler               │ jarw     │
│  8 │ levenshtein                │ lev      │
│  9 │ longest_common_subsequence │ lcsubseq │
│ 10 │ longest_common_substring   │ lcsubstr │
│ 11 │ length                     │ len      │
│ 12 │ lig3                       │ lig      │
│ 13 │ mlipns                     │ mli      │
│ 14 │ overlap                    │ olap     │
│ 15 │ prefix                     │ pre      │
│ 16 │ ratcliff_obershelp         │ rat      │
│ 17 │ roberts                    │ rob      │
│ 18 │ sift4_common               │ scom     │
│ 19 │ sift4_simple               │ ssim     │
│ 20 │ smith_waterman             │ smithw   │
│ 21 │ sorensen_dice              │ soredice │
│ 22 │ suffix                     │ suf      │
│ 23 │ tversky                    │ tv       │
│ 24 │ yujian_bo                  │ ybo      │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │  alias   │
╰────┴────────────────────────────┴──────────╯
```

### str compress
`str compress` will convert nushell values to a string and then compress that string using the specified method with the parameters provided.

#### Usage:

##### Brotli

```nushell
❯ "ABCDEFG" | str compress --brotli
Length: 11 (0xb) bytes | printable whitespace ascii_other non_ascii
00000000:   07 03 80 41  42 43 44 45  46 47 03                   ••×ABCDEFG•
```

##### Flate

```nushell
❯ "ABCDEFG" | str compress --flate
Length: 13 (0xd) bytes | printable whitespace ascii_other non_ascii
00000000:   73 74 72 76  71 75 73 07  00                         strvqus•0
```

##### Zlib

```nushell
❯ "ABCDEFG" | str compress --zlib
Length: 15 (0xf) bytes | printable whitespace ascii_other non_ascii
00000000:   78 5e 73 74  72 76 71 75  73 07 00 07  5b 01 dd      x^strvqus•0•[•×
```

### str decompress
`str decompress` is meant to be the counterpart of `str compress` and decompress whatever it compresses.

#### Usage:

##### Brotli

```nushell
❯ "ABCDEFG" | str compress --brotli | str decompress --brotli
ABCDEFG
```

##### Flate

```nushell
❯ "ABCDEFG" | str compress --flate | str decompress --flate
ABCDEFG
```

##### Zlib

```nushell
❯ "ABCDEFG" | str compress --zlib | str decompress --zlib
ABCDEFG
```

### str dedent
`str dedent` removes common leading whitespace from every line in a text while preserving relative indentation.

#### Usage:

```nushell
❯ "    line 1\n    line 2\n    line 3" | str dedent
line 1
line 2
line 3
```

### str indent
`str indent` adds a specified number of leading spaces to each line in a text.

#### Usage:

```nushell
❯ "line 1\nline 2\nline 3" | str indent 4
    line 1
    line 2
    line 3
```

### str wrap
`str wrap` wraps each line in a text to a specified width.

#### Usage:

```nushell
❯ "This is a long line that needs to be wrapped." | str wrap 10
This is a
long line
that needs
to be
wrapped.
```
