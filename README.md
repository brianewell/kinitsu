A multimedia library standardization and maintenance utility

## Usage

```
kinitsu [OPTIONS] [PATHS]...
```

### Arguments

[PATHS]...  The set of directories to operate on [default: .]

### Options

  -C, --config <CONFIG>
          Path to the global configuration file [default: /etc/kinitsu.toml]
  -c, --directory-config <DIRECTORY_CONFIG>
          Per-directory configuration filename [default: .kinitsu.{type}]
  -t, --types <TYPES>
          Comma-separated ordered list of configuration file formats to check [default: toml,yaml,json,xml]
  -r, --recursive
          Recursively traverse all child directories within the set of provided directory paths
  -s, --symlinks
          Follow symbolic links while traversing directories
  -d, --daemonize
          Run as a background process, continually scanning directories for changes
  -i, --interactive
          Enable interactive mode, showing a summary of changes and prompting users for confirmation of operations on each file
  -p, --preserve
          Preserve original files when performing destructive operations; specifically avoids removing source files and overwriting conflicting destination files
  -n, --noop
          Perform a dry-run ("No-op"); no files will be modified. This is useful in conjunction with the -v option to validate that the expected operations will be performed
  -j, --json
          Produce output in JSON format for scripting or integration with other tools
  -q, --quiet
          Suppress all non-critical output
  -v, --verbose
          Increase verbosity of output
  -h, --help
          Print help
  -V, --version
          Print version

## Examples
