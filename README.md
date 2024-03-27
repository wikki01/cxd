# CXD
Command Executor per Directory (`cxd`) provides a simple interface to save and execute
commands within specific directories. 

## Background
While working in large codebases with several executables, it was common for me to add
custom scripts and Makefiles to each directory. This allowed me to not have to rely on
`bash history` to remember my specific commands. However, after one too many `git-merge`
events overwriting my temporary Makefiles, I wrote `cxd`.

## Use
`cxd` uses a `sqlite` database to save commands and their respective directories to execute
at a later time. Database cache files can be saved and reused across devices. However,
note that you should **NEVER** trust a cache file from an outside source, as `cxd exec`
will execute arbitrary commands from the database.

### Default Command Matching
`cxd` attempts to find the best match for each input using the following hierarchy.

1. Command matches `NAME` and is in `$CWD`
1. Command matches `NAME` and is only match
1. Let user select between all commands matching `NAME`

### Selecting Cache File
By default, `cxd` will attempt to store the cache file in the following locations, and 
fail if unable to construct any.

1. Contents of `-f FILE`
1. `$XDG_CACHE_HOME/cxd.cache`
1. `$HOME/.cache/cxd.cache`

### Pushing
First push a new command with `cxd push`.

```sh
cxd push hello -- echo hello world
```

This will register a command named `hello` with its directory assigned to `$CWD`.

To register a "Global" command, which does not depend on a specific directory, use `-g`. 
```sh
cxd push -g weather -- curl https://wttr.in
```

### Executing
To execute a command from the store, use `cxd exec`. 

```sh
cxd exec hello
```

To only execute specific commands registered globally, from the current directory, or
from a specific directory, use the `-g`, `-c`, `-d DIR` flags respectively.

```sh
cxd push build -- make build
cxd exec -c build
```

### Popping
To remove a command from the store, use `cxd pop`.

```sh
cxd pop hello
```

This has a similar matching scheme to `exec`. Use `-g`, `-c`, and `-d DIR` to control
the matching of `pop`. Otherwise, it will use the default matching scheme.

### Listing
To list all commands in the store, use `cxd list`.

```sh
cxd list
```

### Clearing
To clear all commands in the store, use `cxd clear`.

```sh
cxd clear
```