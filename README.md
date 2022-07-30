# gitcp

Copy files from Git repository to local.

## Usage

```
$ gitcp --help
gitcp 0.1.0
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

### Copy specific file with specific name

```
gitcp owner/repo/foo.txt bar.txt
```

## Configuration

### .gitcpignore

If there is `.gitcpignore` in the copied repository side, it will be used to ignore files.

```
# .gitcpignore
CHANGELOG.md
README.md
```

## Background

A s a tool creator, this can be useful when you want to provide users with some initial template files.

For example, when providing a custom GitHub Action, it's more convenient to explain like this:

> Please run the following command:
>
> ```
> gitcp owner/my-custom-action-template
> ```

than to explain like this:

> Please create a workflow with the following content:
>
> ```yaml
> # .github/workflows/my-custom-action.yml
> ...
> ```
>
> and then put config file like this:
>
> ```yaml
> # my-custom-action-settings.yml
> ...
> ```

I had exactly that experience when I created [github-label-sync-action](https://github.com/r7kamura/github-label-sync-action), so I thought it would be nice to have such a tool, and that's I came to create this.
