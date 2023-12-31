# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Build

- :zap: Remove Vue Options API support for space.
- :building_construction: Fix some build settings.

## [1.3.0] - 2023-12-07

### Documentation

- :memo: Add links in README.

### Refactor

- :recycle: Do some refactor and fix typos.

## [1.2.0] - 2023-11-25

### Miscellaneous Tasks

- :construction_worker: Add cache to CI.
- :construction_worker: Rename workflows.

### Refactor

- :lipstick: Refresh UI.

## [1.1.0] - 2023-11-25

### Features

- :goal_net: Checking for errors in frontend code.

### Miscellaneous Tasks

- :green_heart: Make the CI scripts more specific.
- :see_no_evil: Ignore tsconfig.tsbuildinfo.

## [1.0.1] - 2023-11-25

### Miscellaneous Tasks

- :page_facing_up: Add MIT license.
- :green_heart: Prevent overwriting build of previous releases.

## [1.0.0] - 2023-11-24

### Bug Fixes

- :green_heart: Specify pnpm version.
- :rotating_light: Use different bundle ID.
- :wrench: snake_case to camelCase.
- :wrench: NSIS languages fix.
- :bug: Drag & drop cancel on blurring.
- :rotating_light: Remove unused `use` and rebundant closure.
- :pushpin: Fix pnpm lockfile.
- :lipstick: Merge CSS rules.
- :bug: Fix the color scheme issues.
- :white_check_mark: Successfully extracted signature form packet.
- :bug: Move file name fetching to backend, allowing for platform specific handling of path separators.

### Documentation

- :memo: Add build status in README.
- :memo: Add detailed introduction in README.
- :memo: Add links for VS Codium users.

### Features

- :tada: Initial commit.
- :art: Add icon.
- :sparkles: File selection with drag and drop support.
- :sparkles: Add codec for PEM files.
- :sparkles: Add keypair generation.
- :lipstick: Use Vuetify components.
- :lipstick: Update UI and logo.
- :sparkles: Use rPGP to generate full PGP keys.
- :wrench: Update algorithm preference.
- :sparkles: Add basic signing support.
- :lipstick: Polishing frontend.
- :sparkles: Add light/dark mode switch.
- :sparkles: Remember opened tab on refresh.
- :sparkles: Complex scene of FileSelector.
- :sparkles: Finish key pair generation.
- :poop: Try adding signature verification.
- :sparkles: Introduce `FromFile` trait and uniform the `from_file` function calls.
- :sparkles: Allow user setting password to the keys.
- :sparkles: Refresh SignView.
- :bug: Complete signing feature and start working on verifying.
- :rocket: Finish main functions.

### Miscellaneous Tasks

- :hammer: Try GitHub Actions.
- :green_heart: Add pnpm to GitHub Actions.
- :green_heart: Use pnpm instead of bun
- :wrench: Change Windows language selector.
- :construction_worker: Try fixing GitHub Actions artifacts uploading.
- :pencil2: Fix typo in workflow.

### Refactor

- :fire: Remove unused code and dependencies.
- :wrench: Update config files.
- :recycle: Extract gen_key_pair from write_key_pair.
- :fire: Remove unused icons.
- :adhesive_bandage: Minor fixes.
- :label: Add KeyPair type for better organizing keys.
- :recycle: Add docs and move keygen functions to module `key_pair`.
- :recycle: Move color switcher to a separate component.
- :recycle: Refactor commands into its own module.
- :recycle: Refactor test code for readability.

### Security

- :lock: CVE-2023-46115 fix.

### Build

- :art: Add stylelint related tools.
- :art: Add ESLint
- :bookmark: v0.1.0
- :hammer: Add Makefile for some repetitive work.
- :wrench: Add build config for Vite.

