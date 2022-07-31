# gitcp

[![test](https://github.com/r7kamura/gitcp/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/gitcp/actions/workflows/test.yml)

Copy files from Git repository to local.

## Install

We are planning to add some installers support in the future.

e.g.

- homebrew
- winget
- debian package
- etc.

For now, manually install prebuilt binary from GitHub Releases page:

- https://github.com/r7kamura/gitcp/releases

or install from source via:

```
cargo install gitcp
```

## Usage

```
$ gitcp --help
gitcp 0.3.0
Copy files from Git repository to local.

USAGE:
    gitcp <source> [destination]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>         GitHub repository name (e.g. r7kamura/gitcp)
    <destination>    Path to destination directory [default: .]
```

### Copy all files

Copy all files from https://github.com/owner/repo to current directory.

```
gitcp owner/repo
```

### Copy all files into specific directory

```
gitcp owner/repo tmp/repo
```

### Copy all files from specific ref

```
gitcp owner/repo@v1
```

### Copy specific file

```
gitcp owner/repo/foo.txt
```

### Copy specific files

You can specify [globwalk](https://github.com/Gilnaa/globwalk) powered glob patterns.

```
gitcp owner/repo/{foo,bar}.*
```

## Configuration

### .gitcpignore

If there is `.gitcpignore` in the copied repository side, it will be used to ignore files.

```bash
# .gitcpignore
CHANGELOG.md
README.md
```

## Usecase

This tool can be used not only to simply copy files from any repository, but also as an installer to provide templates for custom GitHub actions.

This is an example to install [github-label-sync-action](https://github.com/r7kamura/github-label-sync-action) into the repository:

```
gitcp r7kamura/github-label-sync-action-template
```

- https://github.com/r7kamura/github-label-sync-action-template

If you always prepare routine files every time you create a repository, I think you can make it easier with `gitcp` by preparing that template.
