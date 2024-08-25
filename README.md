# nu_plugin_strutils

This is a [Nushell](https://nushell.sh/) plugin called "strutils".

## Installing

```nushell
> cargo install --path .
```

## Usage

This plugin implements a some string utilities that are not included in nushell. The first such utility is `str deunicode`. It replaces unicode accented characters with their ASCII counterparts based on the [deunicode crate](https://docs.rs/deunicode/latest/deunicode/).

```nushell
> plugin add ~/.cargo/bin/nu_plugin_strutils
> plugin use strutils
> 'A…C' | str deunicode
A...C
```
