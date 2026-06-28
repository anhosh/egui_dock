# Contribution guide

Before contributing to this project, please follow the guide below.

## Vouch

egui_dock integrates [vouch](https://github.com/mitchellh/vouch) which means that in order to contribute to this
project, you must be vouched by the maintainer.

If you haven't contributed yet and want to get vouched, simply create an issue where you introduce yourself and, in your
own words, say how you would like to contribute.

Only human-written introductions will grant users their vouches. If I notice someone attempt to introduce themselves
with AI-generated text, they will be denounced, i.e., permanently banned from contributing.

Any other breaches of the contribution guide and the AI usage policy will result in a warning, and repeated offense will
result in a denunciation.

For more details, read vouch's [FAQ](https://github.com/mitchellh/vouch/blob/main/FAQ.md).

## AI usage

Please refer to the [AI usage policy](AI_POLICY.md).

## Issues

File an issue only if you want to report a bug, request a feature, or are a first-time contributor and need to get
vouched.

Bug reports should include how to reproduce said bug, if known.

## Pull requests

1. Fork this repository.
2. In your fork create a branch for your changes – do **not** submit directly to `main` or `release-0.x`.
3. Make your changes and open a pull request.
    - If your changes are not complete but e.g. you want feedback on your idea before fully committing to it, open a
      draft PR.
    - Otherwise, feel free to open a regular PR.

**Important:** Open PRs to the `main` branch only if your changes do not introduce incompatibilities with the latest
release of `egui_dock`, a.k.a. breaking changes. All breaking changes should be merged into a special `release-0.x`
branch instead. This is to make it possible to release non-breaking bugfixes without requiring to update to a new
incompatible release.

Before your PR is merged, it needs to be approved by one of the maintainers. To minimize the review time, make sure all
the following requirements are met:

- Your PR is linked to the related issue – if one doesn't exist, create it first.
- Code must be formatted with `cargo fmt` (checked by CI).
- Submitted code must compile and pass all tests (checked by CI).
- All conversations started in a code review must be resolved once you make the requested changes.
- Changelog is up-to-date.
    - Additions to the API must be included in the 'Added' section.
    - Changes to and deletions of existing API must be included in the 'Breaking changes' section.
    - Bugfixes should be included in the 'Fixed' section.
    - Significant changes to behaviour (but not the API) must be included in the 'Changed' section.
    - Deprecated API must be included in the 'Deprecated' section.
- All public API has up-to-date documentation comments.
    - If you override a field of `Style` in `from_egui`, include it in the documentation of this function.
- All major features are included in an example app found in `examples/`.
- The following files and directories are **not** included:
    - .idea/
    - .vs/
    - target/
    - Cargo.lock
