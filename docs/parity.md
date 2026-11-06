# Parity Matrix

Per-capability implementation evidence (guide §II.27.3). Readiness moves only
from verified evidence (a named test or a CI run), never from intention.

| ID | Evidence (test / CI) | Verdict |
|----|----------------------|---------|
| CAP-001 | `workflow_rain_lights_cells_from_the_active_glyph_set`, `workflow_same_seed_is_deterministic_and_different_seeds_diverge` | met |
| CAP-002 | `glyphs::tests::{every_built_in_is_non_empty_and_uniquely_named, cycled_walks_every_set_then_returns_home}` | met |
| CAP-003 | `art::tests::built_in_pack_is_populated_and_well_formed` | met |
| CAP-004 | `workflow_art_piece_is_typed_completed_and_released` | met |
| CAP-005 | `config::tests::{speed_up_and_down…, density_moves_by_tenths…, flicker_ladder…}`, `engine::tests::set_config_re_resolves_glyphs…`; keyboard wiring verified by hand | met |
| CAP-006 | `workflow_theme_save_load_roundtrip`, `config::tests::theme_round_trips_through_a_file` | met |
| CAP-007 | `workflow_user_art_pack_extends_the_built_in_pack`, `art::tests::user_pack_loads_txt_files_and_ignores_the_rest` | met |
| CAP-008 | version constant `ossuary_core::VERSION`; painted in the corner (UI, verified by hand) | met |
| CAP-009 | `RELEASE_NOTES`/`TOKEN_LEDGER` embedded via `include_str!`; overlay verified by hand | met |
| CAP-010 | `crash-report` unit tests (scrubbing, payload) | met |
| CAP-011 | not implemented by design — fully offline | out of scope |
| CAP-012 | not published by design — source distribution only | out of scope |

## Exceeding the inspiration

- Ossuary ships whole hand-typed pictures that assemble from the falling rain,
  which the classic matrix screensaver does not; this is a deliberate
  differentiation, not parity.
