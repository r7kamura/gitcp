# gitcp

Copy files from Git repository to local.

## Usage

### Copy all files

Copy all files from https://github.com/owner/repo to current directory.

```
gitcp owner/repo
```

### Copy all files into specific directory

```
gitcp owner/repo dist
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

If there is `.gitcpignore` in the target repository, it will be used to ignore files.

```
# .gitcpignore
CHANGELOG.md
README.md
```

For example, if we have `.gitcpignore` in the target repository, `CHANGELOG.md` and `README.md` will not be copied. This is mainly a concern for those who provide repositories that are copied by `gitcp`.

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
