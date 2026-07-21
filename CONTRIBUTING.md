# Contributing to Open Device Partnership

The Open Device Partnership project welcomes your suggestions and contributions! Before opening your first issue or pull request, please review our
[Code of Conduct](CODE_OF_CONDUCT.md) to understand how our community interacts in an inclusive and respectful manner.

## Contribution Licensing

Most of our code is distributed under the terms of the [MIT license](LICENSE), and when you contribute code that you wrote to our repositories,
you agree that you are contributing under those same terms. In addition, by submitting your contributions you are indicating that
you have the right to submit those contributions under those terms.

## Other Contribution Information

If you wish to contribute code or documentation authored by others, or using the terms of any other license, please indicate that clearly in your
pull request so that the project team can discuss the situation with you.

## Commit Message

Write clear, meaningful commit messages that follow the Conventional Commits format:

* Structure the subject line as `<type>[optional scope]: <description>`, for example `feat(parser): add ability to parse arrays`.
* Use one of the common types: `feat` (new feature), `fix` (bug fix), `docs` (documentation), `style` (formatting), `refactor`, `perf` (performance), `test`, `build`, `ci`, or `chore`.
* Keep the description in the imperative mood, for example "add ability to parse arrays" rather than "added ability to parse arrays".
* Limit the subject line to 50 characters and do not end it with a period.
* Separate the subject from the body with a blank line and wrap the body at 72 characters.
* Use the body to explain what and why rather than how, and add footers for additional context, such as `Closes #123`.
* Indicate a breaking change with a `!` after the type/scope or a `BREAKING CHANGE:` footer.

## PR Etiquette

* Create a draft PR first
* Make sure that your branch has `.github` folder and all the code linting/sanity check workflows are passing in your draft PR before sending it out to code reviewers.

## Regressions

When reporting a regression, please ensure that you use `git bisect` to find the first offending commit, as that will help us finding the culprit a lot faster.
