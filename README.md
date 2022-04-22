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
Conga collects information about your system, and stores them in a key-value store. Each key, or "tag", consists of a top-level category, sub-categories (if applicable), and a label. For example, `"cpu.cores"` refers to the number of cores the running CPU has. The category is `cpu` and the label is `cores`.

TODO: security example

The values corresponding to each tag will be stored as one of the following types:
 - Integer (i64)
 - Float (f64)
 - Plain Text
 - special case string: `"(null)"`, see below

### Required vs Optional Collectors
Some collectors are optional -- if an optional collector fails to gather its data, it will silently drop the tag from the output.
This may happen if it is depending on some missing userspace command, or it may occur if the collector is not relevant for the given machine (e.g. running on a host versus guest).

If a collector is not optional, then it must be able to gather its information, or else `conga` will terminate early.
This can be bypassed with the `--allow-failures`, and the collector will instead record the string `"(null)"` for the offending tag.
These failures may indicate a bug in `conga`, or it may indicate some system misconfiguration.

NOTE: currently all collectors are mandatory. Optional collectors will be implemented in the future.

### Architecture-specific Collectors
Collectors specific to a particular architecture should be contained within a top-level category, e.g. `"x86.some_subcategory"`.
Exceptions may exist, and should be noted in the table below.
Arch-specific collectors will not run on unsupported architectures, and thus their tags will not be present.

Some core collectors, like `cpu.frequency`, may use different logic for gathering their data.
In most cases, the output format should be unified across all architectures, and exceptions should be noted in the table below.

## Collectors

Name     | Description | Type | Arch | Mandatory
---------|-------------|------|------ | ----------
`cpu.cores` | Number of CPU cores | Integer | *any* | Yes
TODO




