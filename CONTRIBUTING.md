# Contribution guidelines

First off, thank you for considering contributing to {{project-name}}.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Branching Strategy

This project uses [GitFlow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) as its branching strategy.

-   `main`: Contains production-ready code.
-   `develop`: The main development branch. All feature branches are created from here.
-   `feature/*`: For new features. Branched from `develop`.
-   `release/*`: For preparing new production releases. Branched from `develop`.
-   `hotfix/*`: For critical production fixes. Branched from `main`.

## Commit Messages

Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This helps in automating changelog generation and makes the commit history more readable.

A commit message consists of a **header**, a **body**, and a **footer**.

```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

## CI Expectations

The project has a Continuous Integration (CI) pipeline configured to run on every pull request. The CI pipeline runs the following checks:

-   `cargo fmt -- --check`: Checks for code formatting issues.
-   `cargo clippy`: Lints the code for common mistakes.
-   `cargo test`: Runs the test suite.

All checks must pass before a pull request can be merged. For more details on the project's testing strategy and how to run different test suites, please see [`docs/testing.md`](docs/testing.md).

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/{{gh-username}}/{{project-name}}/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/{{gh-username}}/{{project-name}}/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/{{gh-username}}/{{project-name}}
cd {{project-name}}
cargo test
```

### Useful Commands
{% if crate_type == "bin" %}
- Build and run release version:

  ```shell
  cargo build --release && cargo run --release
  ```
{% endif %}
- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
