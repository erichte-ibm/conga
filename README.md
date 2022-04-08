# conga
A utility to collect system runtime configuration data as output as JSON.

## Usage

```
USAGE:
    conga [FLAGS] [OPTIONS]

FLAGS:
    -f, --allow-failures    Don't abort if a collector fails
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -o, --output <output>         Output file path to write to
    -v, --verbose <verbose>...    Increase verbosity
```

Example output:

```
{"cpu.cores":4,"mem.available":19250356,"mem.buff/cache":15200460,"mem.free":5059576,"mem.shared":542100,"mem.total":32827160,"mem.used":12567124,"swap.free":16769276,"swap.total":16777212,"swap.used":7936}
```

## Collector Structure
 - what does the tag.tag.tag... etc mean (hierarchy)
 - types of data output from a collector (enum Text, Integer, ...)
 - implications of a missing tag (not support on arch, optional?)

### arch diff
 - architecture specific tags, most likely under a top-level `"<arch>."` tag
 - gathering behavior and dependencies may differ across arch

## Collectors

Name     | Description | Type | Arch
---------|-------------|------|------
`cpu.cores` | Number of CPU cores | Integer | *any*
TODO




