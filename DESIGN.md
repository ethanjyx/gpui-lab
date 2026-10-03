---
name: GPUI Lab
description: A native scientific visualization workbench with pale instrument chrome and a dark particle field.
colors:
  ink: "#182D3A"
  muted: "#536A77"
  line: "#D5DFE4"
  accent: "#096F76"
  accent-hover: "#075A60"
  paper: "#F4F7F8"
  surface: "#FFFFFF"
  control: "#E8EFF2"
  control-hover: "#DCE7EB"
  plot: "#0F2230"
  plot-label: "#BCD2DA"
  plot-grid: "#39505D"
  particle: "#67D9D1"
  particle-highlight: "#EFB55C"
  row-alternate: "#F7F9FA"
  row-selected: "#DBF0ED"
  row-hover: "#E9F4F3"
  signal-track: "#D8E7E9"
typography:
  title:
    fontFamily: ".SystemUIFont"
    fontSize: "22px"
    fontWeight: 700
  headline:
    fontFamily: ".SystemUIFont"
    fontSize: "20px"
    fontWeight: 600
  panel-title:
    fontFamily: ".SystemUIFont"
    fontSize: "18px"
    fontWeight: 600
  body:
    fontFamily: ".SystemUIFont"
    fontSize: "14px"
  label:
    fontFamily: ".SystemUIFont"
    fontSize: "12px"
  control:
    fontFamily: ".SystemUIFont"
    fontSize: "12px"
    fontWeight: 500
  measurement:
    fontFamily: "Menlo"
    fontSize: "25px"
  mono:
    fontFamily: "Menlo"
    fontSize: "12px"
rounded:
  signal: "4px"
  control: "6px"
  panel: "8px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  panel: "20px"
  gutter: "24px"
  telemetry: "32px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.surface}"
    typography: "{typography.control}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  button-primary-hover:
    backgroundColor: "{colors.accent-hover}"
  button-secondary:
    backgroundColor: "{colors.control}"
    textColor: "{colors.ink}"
    typography: "{typography.control}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  button-secondary-hover:
    backgroundColor: "{colors.control-hover}"
  record-row:
    height: "36px"
    padding: "0 16px"
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
  record-selected:
    backgroundColor: "{colors.row-selected}"
  record-hover:
    backgroundColor: "{colors.row-hover}"
  command-panel:
    backgroundColor: "{colors.surface}"
    rounded: "{rounded.panel}"
    padding: "20px"
    width: "340px"
---

# Design System: GPUI Lab

## Overview

**Creative North Star: "Scientific visualization workbench"**

A restrained native macOS instrument: pale blue-gray chrome frames a dark, animated particle field. Compact controls and measured typography make load, selection, and timing easy to inspect while the drawing remains the visual center. This records the implemented Rust/GPUI prototype, using `src/main.rs` as authority.

**Key Characteristics:**

- Pale instrument surfaces, a dark plot, and precise separators.
- Teal interaction states; turquoise and amber procedural graphics.
- System UI type paired with Menlo for identifiers and measurements.
- Dense desktop controls with visible keyboard hints.

## Colors

The cool neutral palette gives the drawing and active controls distinct roles.

- **Primary:** `accent` marks selected controls, the pause/resume action, list signal bars, the rendered-row count, and the timing trace. `accent-hover` darkens filled controls; selected controls use white text.
- **Data accents:** `particle` and `particle-highlight` are turquoise and amber circles inside `plot`. Every seventh circle uses amber; opacity reflects procedural depth. These are visualization colors, not additional button variants.
- **Neutrals:** `paper` forms the application chrome; `surface` forms the record browser, telemetry strip, and command panel. `ink`, `muted`, and `line` supply primary text, supporting text, and separators. `control` and `control-hover` define quiet actions. The row colors distinguish alternating, selected, and hovered records. Plot labels and grid dots have their own lighter values.

**The State Rule.** Preserve distinct active, inactive, and hover fills. Labels and displayed values explain what each state means.

## Typography

The macOS system UI font carries titles, labels, and controls. Menlo carries changing measurements, zero-padded record identifiers, signal values, and keyboard equivalents. Use the frontmatter hierarchy; the app title is bold, section titles semibold, and controls medium. Supporting labels remain compact. Plot status and the application footer use a smaller size (11px); the telemetry introduction uses semibold type (13px).

**The Measurement Rule.** Keep values and identifiers monospaced, preserve units beside measurements, and avoid enlarging utility labels into competing headlines.

## Layout

Desktop only. The native window opens at 1280 × 840 logical pixels and enforces a 1080 × 680 minimum. Keep the native title bar. The application header is fixed height (66px). The central workspace uses a flexible left visualization column and a fixed record browser (320px), separated by a thin rule. The plot grows with the window and keeps a minimum height (240px). Reserve internal space for its top instruction/status and bottom shortcut hint so particles do not obscure them.

Controls sit immediately below the plot: particle load at left, scene and grid at right. A fixed telemetry strip (130px) spans the window above the footer (34px). The trace takes the space remaining after the fixed measurement groups. Vertical resizing changes the list viewport and plot height; it does not create a mobile layout or stack the columns. The list scrolls independently.

Use the documented spacing scale: small gaps group related labels and controls; panel and gutter spacing separate working regions. GPUI's default rem size is 16px; the frontmatter resolves its spacing and corner helpers to logical pixels.

## Elevation & Depth

The interface is flat and uses no shadows. Surface tone, one-pixel separators, and the dark plot establish the hierarchy. The command panel overlays the workspace on a white surface with a thin border, without a dimmed backdrop. Particle opacity provides depth only within the drawing.

## Shapes

Working regions meet at square edges. Modestly rounded buttons and the command panel soften the controls without turning the workspace into a card grid. Signal bars use the smaller radius. The custom canvas clips its content; particles are circles drawn from rounded quads.

## Components

- **Buttons and selectors:** compact filled rectangles with the frontmatter padding and radius. Active load, scene, and grid buttons use the primary treatment; quiet actions use the secondary treatment. Hover changes fill immediately. Pause/resume stays primary and changes its label. The implementation dispatches keyboard actions at the window level; it does not establish a per-button focus-ring system.
- **Particle field:** a dark continuous plot, optional sparse dot grid (48px spacing), a top-left instruction, top-right LIVE/PAUSED status, and bottom-left shortcuts. Pointer movement displaces nearby circles; clicking switches Orbit/Wave. Load choices are 1,000, 10,000, and 30,000. Animation follows frame callbacks, and pause retains the scene.
- **Record browser:** a white panel with jump controls, a pale column header, alternating fixed-height rows, a selected-row tint, small signal bars, and a footer. The 100,000 records are labeled synthetic. Display the range size requested by the virtualized renderer rather than a hard-coded visible count. Selection and scrolling remain useful while the animation is paused.
- **Telemetry:** three Menlo measurements with adjacent units and muted labels, followed by a thin teal interval trace (1.5px). The rolling sample window contains up to 180 callback intervals. Keep the 16.7ms reference identified, retain measurements on pause, and clear history when particle load changes. Its label describes callback timing rather than GPU execution time.
- **Keyboard controls panel:** a compact top-right overlay containing action names, right-aligned Menlo shortcuts, and a quiet close button. It opens with the header control or Cmd K and closes with Escape or its close action. There is no text input or search field.

## Do's and Don'ts

- **Do** preserve the native desktop shell, compact control density, and flexible plot beside the fixed-width record browser.
- **Do** use the same control treatments and monospaced measurement roles when extending the workbench.
- **Do** check the default and minimum window sizes, including paused, selected-row, and command-panel states.
- **Do** keep synthetic data labels, measurement units, and timing limitations visible.
- **Don't** replace the plot with imagery or add decorative cards, gradients, shadows, or unrelated accent colors.
- **Don't** report callback rate as GPU execution speed or a controlled comparative benchmark.
- **Don't** treat the prototype's keyboard bindings as evidence of complete platform accessibility.
