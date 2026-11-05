# Governance

Ossuary is a small, spare-time project. This document says plainly how it is run.

## Roles

- **Maintainer.** One person, who reviews and merges changes, cuts releases and
  sets direction. The maintainer does not write code a contributor could write;
  the maintainer's job is goals, priorities and "does this work?".
- **Contributors.** Anyone who opens an issue or a pull request, under the
  terms in `CONTRIBUTING.md`.

## How decisions are made

- Small changes: the maintainer decides in the pull request.
- Anything that changes scope or direction: an issue first, so the trade-off is
  visible before work starts.
- Significant, hard-to-reverse decisions are recorded in `DECISIONS.md`.

## Releases

Releases are progressive and semantic (`0.1.1`, `0.2.0`, …), each with honest
release notes (Added / Changed / Fixed / Known limitations), published to GitHub
Releases and embedded in the app. There is no "big bang" launch.

## Response expectations

We aim to reply to issues and pull requests within seven days. That is a real
commitment for a spare-time project, not a guarantee of an immediate fix.

## Changes to this document

Changes to governance are proposed in an issue and applied by the maintainer.
