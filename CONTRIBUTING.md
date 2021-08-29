# Contributing

Do you want to contribute? We'd love that.

Our goal is to have mostly projects that are stable and useful to many users.

## How?

The easiest way is to go to https://github.com/rust-unofficial/awesome-rust/blob/master/README.md and click on the "pen" icon in the upper right corner. Make the changes to the file and follow the instructions to create a pull request.

If you want to add an entry to the `README.md` please consider this:

- is the entry valuable to people trying to get things done in Rust?
    * In order to make this objective, the entry needs to either have at least 50 stars on Github, 2000 downloads on crates.io, or an equivalent level of other popularity metrics (which should be specified in the PR). The maintainers of this repo are not responsible for making your project popular, only for making more people aware of those projects. We don't want to have to pick and choose favourites, and so are using metrics like this to make our lives easier as maintainers.
- if you want to add something, please use the template `[ACCOUNT/REPO](https://github.com/ACCOUNT/REPO) [[CRATE](https://crates.io/crates/CRATE)] — DESCRIPTION`
    * if you've not published your crate to `crates.io` remove the `[[CRATE](...)]` part.
    * if you have a CI build, please add the build badge. Put the image after the description, separated by a space. Please make sure to add the branch information to the image:
        * example for Travis: `[![build badge](https://api.travis-ci.com/XXX/CRATE.svg?branch=master)](https://app.travis-ci.org/github/XXX/CRATE)`
        * for Github actions please see [adding-a-workflow-status-badge](https://docs.github.com/en/actions/managing-workflow-runs/adding-a-workflow-status-badge)
- please pay attention to the alphabetical ordering

## Removing projects

We don't remove projects unless they are outright broken or pronounced deprecated by another project or by its author.
