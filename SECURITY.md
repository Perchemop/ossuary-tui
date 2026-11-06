# Security Policy

## Reporting a vulnerability

Please report security issues **privately** through GitHub's Private
Vulnerability Reporting (the repository's **Security** tab → "Report a
vulnerability"), rather than in a public issue.

We aim to acknowledge reports within seven days. This project is maintained in
spare time, so that is a firm expectation rather than a guaranteed instant fix.
Coordinated disclosure: after a report is confirmed, a fix is prepared and a
disclosure date is agreed with the reporter. Reporters are credited unless they
ask not to be.

## Scope

In scope: the Ossuary source and the crates in this repository, the theme and
user-art file handling, the crash-capture path, and the CI workflow.

Out of scope: the user's terminal and operating system.

## Security properties of this project

- **No network at all.** Ossuary makes no network requests. There is no update
  check, no telemetry and no crash auto-submission. The crash helper only writes
  a local log.
- **No embedded credentials.** The app ships no tokens or keys.
- **Scrubbed logs.** Crash logs have home paths and credential-looking strings
  removed before they are written.
- **Local-first data.** The theme file and user art are plain files on the
  user's own disk.
- **Small supply chain.** A deliberately small dependency set; the crates are
  audited by `cargo-deny` in CI.

## Scans in CI

Every pull request runs a secret scan and a personal-data scan as blocking steps
of the `check` workflow, alongside `cargo-deny` (licences, advisories, bans,
sources).
