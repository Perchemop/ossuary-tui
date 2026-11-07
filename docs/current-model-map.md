# Current Model Map

Which free model is assigned to which role for the current session. Updated
whenever the available free models change (guide §5.2).

Guide synchronisation (§12.3): this ruleset was last diffed against
`~/guide.md` on **2026-11-07**; §12.2 parallel staff simulation adopted that
day (D-009).

## Provider order (guide §5)

1. **OpenCode free models** (default).
2. **NVIDIA free models** — used automatically when OpenCode free is exhausted
   (rate limits, quota or unavailability), via the configured API key.

No local model hosting (Ollama or similar) is used. Only the provider changes
between the two tiers; process, roles, vertical-slice discipline, CI-first rule
and tracking files stay identical.

## OpenCode free (discovered 2026-11-06)

Live catalog at session start: `crouton-alpha-free`, `pulse-alpha-free`,
`flint-40b-free`, `nemotron-3.5-super-vl-free`, `beam-free`,
`mimo-v2.6-flash-free`, `deepseek-v4.1-flash-free`, `glm-5.3-flash-free`,
`big-pickle` (all zero-cost).

| Role | Model |
|---|---|
| Orchestrator / lead (plans, never edits) | best available free model in-session; `deepseek-v4.1-flash-free#high` is the current coder fallback |
| Domain dev (pure Rust types/logic/tests) | `opencode/deepseek-v4.1-flash-free#high` |
| Terminal UI dev (crossterm shell) | `opencode/flint-40b-free#high` |
| Fixer (build-gate failures) | `opencode/flint-40b-free#high` |
| Reviewer (read-only review) | `opencode/glm-5.3-flash-free#high` |
| Test writer | `opencode/mimo-v2.6-flash-free` |
| Docs writer | `opencode/beam-free#high` |
| Designer (visual/UX review) | `opencode/nemotron-3.5-super-vl-free#high` (vision-capable) |

Model IDs are not load-bearing: they change over time, so the orchestrator
re-discovers the live catalog at session start and reassigns the best fit per
role (guide §5.2).

## NVIDIA free (fallback tier, not currently active)

Discovered via the models tool with `provider = "nvidia"` when the free tier is
exhausted; representative ids: `nvidia/deepseek-ai/deepseek-v4.1-flash`,
`nvidia/z-ai/glm-5.3-flash`, `nvidia/z-ai/glm-5.3`,
`nvidia/nvidia/nemotron-3.5-super-vl-120b-a12b`. Guard/vision-only models are
excluded from roles.

## MCP and tooling status (2026-11-06)

Context7 is available for library/framework documentation and was used to
confirm the `crossterm` API surface. GitHub operations (repository creation,
PRs, runs, checks, releases) run through the authenticated `gh` CLI rather than
a GitHub MCP server, which is not present in this environment's tool catalog.
The effect guide §D.1 asks for is met by `gh`. Recorded here as an adaptation
rather than assumed.

## Research method (guide §2.4)

Public research for `docs/inspiration.md` used the browse/fetch capability of
the working model to read public pages, then wrote original behavioural
summaries with URLs only — no copied text, screenshots or assets. The human gate
(guide §8.4) applies before any dependent implementation.

## Parallel team composition (guide §12.2)

Each vertical slice is staffed by 6–8 agents with a persistent supervisor across
slices. Roles map to the harness subagents as follows; the orchestrator assigns
each role the best live free model at slice start, per the table above.

| Guide role | Subagent | Model source |
|---|---|---|
| Supervisor / tech lead | orchestrating session | best reasoning/tool-calling free model in-session |
| Lead developer (core) | `domain-dev` | coder role model |
| UI developer (shell) | `ui-dev` | coder role model |
| Reviewer | `reviewer` | reviewer role model |
| QA tester / bug hunter | `test-writer` | tester role model |
| Explorer / researcher | `explore` | fastest free model |
| Docs writer | `docs-writer` | docs role model |
| Fixer | `fixer` | coder role model |
| Rotating specialist | `general` | per-task selection |

Rules: workers never ask the human and never push/merge/tag; the supervisor is
the sole user of the question tool and the sole pusher/merger. When free-model
quota is exhausted mid-slice, the team shrinks to the critical path first
(sequential) and non-blocking parallel work resumes after quota resets — the
exhaustion is recorded here and in `docs/process-feedback.md`.

