# Verification

Verified locally on macOS 26.3, Apple Silicon, using Rust 1.89 and GPUI 0.2.2.

- Native application built successfully with the pinned dependency lockfile.
- All three unit tests passed: bounded/invalid timing samples, slow-frame mean/P95, and normalized scene coordinates at each particle count.
- Formatting check and app manifest validation passed.
- Packaged executable hash matches the final build.
- Live mouse checks: particle load selection, first and last record buttons.
- Live keyboard checks: pause/resume, scene switch, load change, row-50,000 jump, controls panel open/dismiss.
- Live scrolling: the 100,000-record viewport advances while rendering only the requested visible range.
- Live resizing: verified at the 1080×680 minimum content size; paused list count updates to six rows.
- Empty-history check: changing load while paused displays dashes and “No samples · resume to measure.”

## Independent finish review: PASS

| Finding | Resolution | Evidence |
| --- | --- | --- |
| Particles overlap instructions at minimum height | Reserved space above and below the field | `screenshots/compact.png` |
| Rendered-row count remains stale after paused resize | Count updates with the viewport | `screenshots/compact.png` |
| Empty timing history appears as measured zeros | Em dashes and correct empty-state explanation | `screenshots/empty-timing.png` |

The independent review inspected source and screenshots; live interactions and tests were performed by the primary agent. No production screen-reader verification or controlled benchmark comparison was performed. See README.md for measurement definitions and prototype limitations.
