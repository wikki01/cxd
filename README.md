# CXD
Command Executor per Directory (`cxd`) provides a simple interface to save and execute
commands within specific directories. 

## Background
While working in large codebases with several executables, it was common for me to add
custom scripts and Makefiles to each directory. This allowed me to not have to rely on
`bash history` to remember my specific commands. However, after one too many `git-merge`
events overwriting my temporary Makefiles, I wrote `cxd`.

## Installation

```sh
cargo install cxd
```

## Use
`cxd` uses a `sqlite` database to save commands and their respective directories to execute
at a later time. Database cache files can be saved and reused across devices. However,
note that you should **NEVER** trust a cache file from an outside source, as `cxd`
will execute arbitrary commands from the database.

### Selecting Cache File
By default, `cxd` will attempt to store the cache file in the following locations, and 
fail if unable to construct any.

1. Contents of `-f FILE` or `--file FILE`
1. `$CXD_CACHE_DIR/cxd.cache`
1. `$XDG_CACHE_HOME/cxd.cache`
1. `$HOME/.cache/cxd.cache`

### Adding a Command
To add a command to the database, use `cxd --add <NAME> <CMD> [ARG]...`. 

```sh
cxd --add hello echo Hi there!
```

This will register a command named `hello` that prints out a greeting.

If you'd like the command to swap directories back to your current `$CWD` before invoking 
the command, pass the `--cwd` flag.

```sh
cxd --add --cwd build cargo build
```

Similarly, you can set a specific working directory with `--dir <DIR>`.

```sh
cxd --add --dir /src/cxd build cargo build
```

If specific environment variables must be set, use `--env <KEY>=<VALUE>`.

```sh
cxd --add --env SOME_ENV=hi hello printenv SOME_ENV
```

### Executing a command
To execute a command from the database, use `cxd <CMD>`. 

```sh
cxd hello
```

### Removing a command
To remove a command from the database, use `cxd --remove <CMD>`.

```sh
cxd --remove hello
```

### Listing
To list all commands in the database, use `cxd --list`.

```sh
cxd --list
```

### Clearing
To clear all commands in the database, use `cxd --clear`.

```sh
cxd --clear
```

## Tips
### Using multiple cache files
It can be useful to segment cache files for specific commands. 
A simple and ergonomic way to do this is to set aliases.

For example, imagine you want two separate cache files, one for play and one for work.

Then you can add the following to your `.bashrc`, `.zshrc`, or equivalent.
```sh
alias build="cxd -f .cache/cxd.build.cache"
alias play="cxd -f .cache/cxd.play.cache"
```

This allows you to run `cxd` with each cache file as if it were two different commands, `build` and `play`.
