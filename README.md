git-brws
========

`git-brws` is a command line tool to open a repository, file, commit or diff in your web browser from command line.

Features:

- Opens a page of
  - Repository (e.g. https://github.com/rhysd/git-brws)
  - File (e.g. https://github.com/rhysd/git-brws/blob/master/Cargo.toml)
  - Commit (e.g. https://github.com/rhysd/git-brws/commit/60024ab1280f9f10423b22bc708f3f6ef97db6b5)
  - Diff (e.g. https://github.com/rhysd/git-brws/compare/60024ab1280f9f10423b22bc708f3f6ef97db6b5...e3c18d0d50252112d37bde97061370204b3cdab7)
- Supports below services
  - [GitHub](https://github.com)
  - [Bitbucket](https://bitbucket.org)
  - [GitHub Enterprise](https://enterprise.github.com/home) (Will be supported soon)
  - [GitLab](https://about.gitlab.com/) (Will be supported soon)
- Prefers commit-specific page URL

## Installation

- With [cargo](https://crates.io/)

```
$ cargo install git-brws
```

- As single binary

Not yet (I'll upload soon)

## Usage

```
Usage: target/debug/git-brws [Options] {Args}

Options:
    -r, --repo REPO     Shorthand format (user/repo, service/user/repo) or
                        remote name (e.g. origin) or Git URL you want to see
    -b, --branch BRANCH Branch name of the repository
    -d, --dir PATH      Directory path to your repository
    -u, --url           Output URL to STDOUT instead of opening in browser
    -h, --help          Print this help
    -v, --version       Show version
```

## Usage Examples

### Open a repository page

- Open current repository page

```
$ git brws
```

- Open 'develop' branch

```
$ git brws -b develop
```

- Open 'origin' remote of 'develop' branch

```
$ git brws -r origin -b develop
```

- Open 'upstream' remote of 'develop' branch of @rhysd's 'Shiba' repository

```
$ git brws upstream develop -repo=rhysd/Shiba
```

- Open specific file of current branch of current remote

```
$ git brws ./some-file.go
```

- Open specific line of the file at current branch of current remote

```
$ git brws ./some-file.go#L123
```

### Open a specific commit page

- Open `HEAD` page of current repository

```
$ git brws HEAD
```

### Show a specific diff page

- Show diff between `HEAD` and `HEAD~3`

```
$ git brws HEAD..HEAD~3
```

- Show diff between `60024ab` and `113079b`

```
$ git brws 60024ab..113079b
```

### Open issues/PRs (NOT YET)

- Open issue `#12`

```
$ git brws issues/12
```

- Open PR `#12`

```
$ git brws pr/12
```

## Related Projedts

- [hub browse](https://hub.github.com/)
- [git open](https://github.com/paulirish/git-open)
- [open-browser-github.vim](https://github.com/tyru/open-browser-github.vim)

## License

Distributed under [the MIT license](LICENSE).

## TODOs

Please see [the project page](https://github.com/rhysd/git-brws/projects).

## Development

```sh
cargo install cargo-watch
```

```sh
# Watch and build sources/tests automatically
cargo watch
```
