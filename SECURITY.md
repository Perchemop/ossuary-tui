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

In scope: the Ossuary binary and the crates in this repository, the theme and
user-art file handling, the optional update check and crash-capture paths, and
the CI/release workflows.

Out of scope: the user's terminal and operating system, and any service the user
connects to voluntarily.

## Security properties of this project

- **No embedded credentials.** The app ships no tokens or keys. The update check
  reads only public release information from GitHub.
- **No automatic submission.** Crash capture writes a **scrubbed** local log
  (home paths and credential-looking strings are removed) and never sends
  anything on its own.
- **No network unless asked.** The only network call is the user-initiated
  update check; it fails soft when offline.
- **Local-first data.** The theme file and user art are plain files on the
  user's own disk.

## Scans in CI

Every pull request runs a secret scan and a personal-data scan as blocking
steps of the `check` workflow, and release assets are scanned before they are
published.
