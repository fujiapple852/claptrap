# Claptrap Agent Guidelines

This repository follows the guidance below when making changes.

## Development commands

- Test the code with `cargo test`.
- Format Rust code with `cargo fmt --all`.
- Do not attempt to format non-Rust code
- Lint with `cargo clippy --workspace --all-features --tests -- -Dwarnings`.
- If test output changes update snapshots with `cargo test && cargo insta review`.
- If the `Dockerfile` changes, build it locally using `docker build . -t claptrap:dev`.

## Commit messages

- Use the Conventional Commits format:
  `<type>: <description>` where `<type>` is one of
  `feat`, `fix`, `chore`, `docs`, `style`, `refactor`, `test`, `build`, `ci`, or `revert`.
- Use backquotes for file names and code items in the description.
- For documentation fixes use `docs: fix <description>`.
- Prefer small, focused commits. For larger changes, use multiple commits with clear messages.

## Recommendations

- Run test, format and clippy before submitting a pull request and ensure all CI checks pass.
- Keep documentation and examples in sync with code changes.
- Use feature branches for separate tasks.
- Open issues and pull requests through GitHub for discussion and review.
- Always rebase your branch before when editing an open pull request to keep the history clean.
