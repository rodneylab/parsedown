Template repo for syncing Rust CI GitHub workflows and other config.

Repo is intended for use with Rust projects, so most CI GitHub workflows run on
this template will fail; they need a project with Rust code, tests so on, configured to pass.

Based on
[process described by Jon Gjengset in this Setting up CI stream](https://www.youtube.com/watch?v=xUH-4y92jPg)

## Usage

From a Rust project run:

```shell
git remote add ci https://github.com/rodneylab/rust-ci-conf
git fetch ci
git merge --allow-unrelated ci/main
```

This will clone the history of this repo and merge it with yours. You can also
merge updates to these templates (by running the `git fetch ci` & `git merge`
steps above again).
