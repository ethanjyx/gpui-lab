/*
THESIS: Put a responsive native interface under visible drawing load.
OWN-WORLD: Scientific visualization workbench; pale instrument chrome, an ink-blue
plot, turquoise and amber data, compact native controls, tabular measurements.
STORY: Move through the field, raise the primitive count, scroll a huge list,
then inspect measured frame intervals. All sample list records are synthetic.
FIRST VIEWPORT: Broad GPU canvas beside a narrow virtualized record browser;
load controls below the canvas; measured timing and its trace along the bottom.
FORM: Interactive visualization workbench, direction 6, seed aa1e5b8b.
FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, and DESIGN.md
*/
mod simulation;
use gpui::{prelude::*, *};
use simulation::{FrameStats, Scene, position};
use std::{cell::Cell, rc::Rc, time::Instant};

const INK: u32 = 0x182D3A;
const MUTED: u32 = 0x536A77;
const LINE: u32 = 0xD5DFE4;
const ACCENT: u32 = 0x096F76;
const PAPER: u32 = 0xF4F7F8;
const PLOT: u32 = 0x0F2230;
const ROWS: usize = 100_000;

actions!(
    lab,
    [
        TogglePause,
        ToggleScene,
        ToggleGrid,
        RaiseLoad,
        LowerLoad,
        ToggleCommands,
        Dismiss,
        JumpMiddle,
        FirstRow,
        LastRow,
        NextRow,
        PreviousRow,
        Quit
    ]
);

struct Lab {
    focus: FocusHandle,
    count: usize,
    scene: Scene,
    running: bool,
    grid: bool,
    commands: bool,
    clock: f32,
    last_frame: Option<Instant>,
    frame_pending: bool,
    stats: FrameStats,
    cursor: Option<Point<Pixels>>,
    scroll: UniformListScrollHandle,
    visible: Rc<Cell<usize>>,
    selected: usize,
}

fn label(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(12.))
        .text_color(rgb(MUTED))
        .child(text.into())
}
fn mono(text: impl Into<SharedString>) -> Div {
    div()
        .font_family("Menlo")
        .text_size(px(12.))
        .child(text.into())
}
fn button(
    id: &'static str,
    title: impl Into<SharedString>,
    selected: bool,
    cx: &mut Context<Lab>,
    callback: impl Fn(&mut Lab, &mut Context<Lab>) + 'static,
) -> Stateful<Div> {
    div()
        .id(id)
        .px_3()
        .py_2()
        .rounded_md()
        .cursor_pointer()
        .text_size(px(12.))
        .font_weight(FontWeight::MEDIUM)
        .bg(rgb(if selected { ACCENT } else { 0xE8EFF2 }))
        .text_color(rgb(if selected { 0xFFFFFF } else { INK }))
        .hover(move |s| s.bg(rgb(if selected { 0x075A60 } else { 0xDCE7EB })))
        .on_click(cx.listener(move |this, _, _, cx| {
            callback(this, cx);
            cx.notify();
        }))
        .child(title.into())
}

impl Lab {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus);
        Self {
            focus,
            count: 10_000,
            scene: Scene::Orbit,
            running: true,
            grid: true,
            commands: false,
            clock: 0.,
            last_frame: None,
            frame_pending: false,
            stats: FrameStats::default(),
            cursor: None,
            scroll: UniformListScrollHandle::new(),
            visible: Rc::new(Cell::new(0)),
            selected: 0,
        }
    }
    fn pause(&mut self) {
        self.running = !self.running;
        self.last_frame = None;
    }
    fn set_count(&mut self, count: usize) {
        self.count = count;
        self.stats = FrameStats::default();
    }
    fn jump(&mut self, index: usize) {
        self.selected = index.min(ROWS - 1);
        self.scroll
            .scroll_to_item_strict(self.selected, ScrollStrategy::Center);
    }
    fn toggle_pause(&mut self, _: &TogglePause, _: &mut Window, cx: &mut Context<Self>) {
        self.pause();
        cx.notify();
    }
    fn toggle_scene(&mut self, _: &ToggleScene, _: &mut Window, cx: &mut Context<Self>) {
        self.scene = if self.scene == Scene::Orbit {
            Scene::Wave
        } else {
            Scene::Orbit
        };
        cx.notify();
    }
    fn toggle_grid(&mut self, _: &ToggleGrid, _: &mut Window, cx: &mut Context<Self>) {
        self.grid = !self.grid;
        cx.notify();
    }
    fn raise(&mut self, _: &RaiseLoad, _: &mut Window, cx: &mut Context<Self>) {
        self.set_count(if self.count < 10_000 { 10_000 } else { 30_000 });
        cx.notify();
    }
    fn lower(&mut self, _: &LowerLoad, _: &mut Window, cx: &mut Context<Self>) {
        self.set_count(if self.count > 10_000 { 10_000 } else { 1_000 });
        cx.notify();
    }
    fn toggle_commands(&mut self, _: &ToggleCommands, _: &mut Window, cx: &mut Context<Self>) {
        self.commands = !self.commands;
        cx.notify();
    }
    fn dismiss(&mut self, _: &Dismiss, _: &mut Window, cx: &mut Context<Self>) {
        self.commands = false;
        cx.notify();
    }
    fn first(&mut self, _: &FirstRow, _: &mut Window, cx: &mut Context<Self>) {
        self.jump(0);
        cx.notify();
    }
    fn last(&mut self, _: &LastRow, _: &mut Window, cx: &mut Context<Self>) {
        self.jump(ROWS - 1);
        cx.notify();
    }
    fn middle(&mut self, _: &JumpMiddle, _: &mut Window, cx: &mut Context<Self>) {
        self.jump(49_999);
        cx.notify();
    }
    fn next(&mut self, _: &NextRow, _: &mut Window, cx: &mut Context<Self>) {
        self.jump((self.selected + 1).min(ROWS - 1));
        cx.notify();
    }
    fn previous(&mut self, _: &PreviousRow, _: &mut Window, cx: &mut Context<Self>) {
        self.jump(self.selected.saturating_sub(1));
        cx.notify();
    }

    fn schedule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.running || self.frame_pending {
            return;
        }
        self.frame_pending = true;
        let entity = cx.entity().downgrade();
        window.on_next_frame(move |_, cx| {
            let _ = entity.update(cx, |this, cx| {
                this.frame_pending = false;
                if !this.running {
                    return;
                }
                let now = Instant::now();
                if let Some(last) = this.last_frame {
                    let dt = now.duration_since(last).as_secs_f32();
                    this.stats.push(dt * 1000.);
                    this.clock += dt.min(0.1);
                }
                this.last_frame = Some(now);
                cx.notify();
            });
        });
    }

    fn field(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (count, time, scene, grid, cursor) =
            (self.count, self.clock, self.scene, self.grid, self.cursor);
        div()
            .id("particle-field")
            .relative()
            .flex_1()
            .min_h(px(240.))
            .w_full()
            .bg(rgb(PLOT))
            .overflow_hidden()
            .on_mouse_move(cx.listener(|this, ev: &MouseMoveEvent, _, cx| {
                this.cursor = Some(ev.position);
                cx.notify();
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.scene = if this.scene == Scene::Orbit {
                        Scene::Wave
                    } else {
                        Scene::Orbit
                    };
                    cx.notify();
                }),
            )
            .on_hover(cx.listener(|this, hovered, _, cx| {
                if !hovered {
                    this.cursor = None;
                    cx.notify();
                }
            }))
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        let w = (f32::from(bounds.size.width) - 48.).max(1.);
                        let h = (f32::from(bounds.size.height) - 96.).max(1.);
                        let ox = f32::from(bounds.origin.x) + 24.;
                        let oy = f32::from(bounds.origin.y) + 48.;
                        window.with_content_mask(Some(ContentMask { bounds }), |window| {
                            // A single quad batch preserves this decorative field's paint order
                            // without inserting every particle into GPUI's bounds tree.
                            window.paint_layer(bounds, |window| {
                                if grid {
                                    for x in (0..w as usize).step_by(48) {
                                        for y in (0..h as usize).step_by(48) {
                                            window.paint_quad(fill(
                                                Bounds::new(
                                                    point(px(ox + x as f32), px(oy + y as f32)),
                                                    size(px(1.), px(1.)),
                                                ),
                                                rgb(0x39505D),
                                            ));
                                        }
                                    }
                                }
                                for i in 0..count {
                                    let (nx, ny, depth) = position(i, count, time, scene);
                                    let mut x = ox + nx * w;
                                    let mut y = oy + ny * h;
                                    if let Some(mouse) = cursor {
                                        let dx = x - f32::from(mouse.x);
                                        let dy = y - f32::from(mouse.y);
                                        let d = (dx * dx + dy * dy).sqrt();
                                        if d < 110. && d > 0.1 {
                                            let force = (1. - d / 110.).powi(2) * 42.;
                                            x += dx / d * force;
                                            y += dy / d * force;
                                        }
                                    }
                                    let radius = if count > 10_000 {
                                        1.1
                                    } else {
                                        1.3 + depth * 1.1
                                    };
                                    let mut color = if i % 7 == 0 {
                                        rgb(0xEFB55C)
                                    } else {
                                        rgb(0x67D9D1)
                                    };
                                    color.a = 0.35 + depth * 0.65;
                                    window.paint_quad(quad(
                                        Bounds::new(
                                            point(px(x), px(y)),
                                            size(px(radius * 2.), px(radius * 2.)),
                                        ),
                                        px(radius),
                                        color,
                                        px(0.),
                                        transparent_black(),
                                        Default::default(),
                                    ));
                                }
                            });
                        });
                    },
                )
                .size_full(),
            )
            .child(
                div()
                    .absolute()
                    .left_5()
                    .top_4()
                    .text_size(px(12.))
                    .text_color(rgb(0xBCD2DA))
                    .child("Move your pointer through the field"),
            )
            .child(
                div()
                    .absolute()
                    .left_5()
                    .bottom_4()
                    .text_size(px(11.))
                    .text_color(rgb(0xBCD2DA))
                    .child("Click canvas to switch scene  ·  Space to pause"),
            )
            .child(
                div()
                    .absolute()
                    .right_5()
                    .top_4()
                    .font_family("Menlo")
                    .text_size(px(11.))
                    .text_color(rgb(0xBCD2DA))
                    .child(if self.running { "LIVE" } else { "PAUSED" }),
            )
    }

    fn records(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let visible = self.visible.clone();
        div()
            .w(px(320.))
            .flex_shrink_0()
            .flex()
            .flex_col()
            .bg(rgb(0xFFFFFF))
            .border_l_1()
            .border_color(rgb(LINE))
            .child(
                div()
                    .p_5()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(18.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("100,000 rows. Keep scrolling."),
                    )
                    .child(label("Only the visible rows become UI elements."))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .mt_2()
                            .child(button("first", "First", false, cx, |s, _| s.jump(0)))
                            .child(button("middle", "50,000", false, cx, |s, _| s.jump(49_999)))
                            .child(button("last", "Last", false, cx, |s, _| s.jump(ROWS - 1))),
                    ),
            )
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(PAPER))
                    .border_y_1()
                    .border_color(rgb(LINE))
                    .flex()
                    .justify_between()
                    .child(label("SYNTHETIC RECORDS"))
                    .child(label("SIGNAL")),
            )
            .child(
                uniform_list(
                    "records",
                    ROWS,
                    cx.processor(move |this, range: std::ops::Range<usize>, window, cx| {
                        let new_visible = range.len();
                        if visible.replace(new_visible) != new_visible {
                            cx.on_next_frame(window, |_, _, cx| cx.notify());
                        }
                        range
                            .map(|i| {
                                let selected = this.selected == i;
                                let value = (i * 73 + 19) % 100;
                                div()
                                    .id(("record", i))
                                    .w_full()
                                    .h(px(36.))
                                    .px_4()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .bg(rgb(if selected {
                                        0xDBF0ED
                                    } else if i % 2 == 0 {
                                        0xFFFFFF
                                    } else {
                                        0xF7F9FA
                                    }))
                                    .hover(|s| s.bg(rgb(0xE9F4F3)))
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.selected = i;
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .items_center()
                                            .child(
                                                mono(format!("{:06}", i + 1))
                                                    .text_color(rgb(MUTED)),
                                            )
                                            .child(div().text_size(px(12.)).child(
                                                ["Particle", "Emitter", "Vector", "Sample"][i % 4],
                                            )),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .w(px(46.))
                                                    .h(px(4.))
                                                    .bg(rgb(0xD8E7E9))
                                                    .rounded_sm()
                                                    .child(
                                                        div()
                                                            .w(px(value as f32 * 0.46))
                                                            .h_full()
                                                            .bg(rgb(ACCENT))
                                                            .rounded_sm(),
                                                    ),
                                            )
                                            .child(mono(format!("{:02}", value)).w(px(18.))),
                                    )
                            })
                            .collect()
                    }),
                )
                .track_scroll(self.scroll.clone())
                .w_full()
                .flex_1()
                .min_h(px(0.)),
            )
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(LINE))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        mono(format!(
                            "{} rows rendered / 100,000 total",
                            self.visible.get()
                        ))
                        .text_color(rgb(ACCENT)),
                    )
                    .child(label(format!(
                        "Selected #{:06}  ·  Up / Down to move",
                        self.selected + 1
                    ))),
            )
    }

    fn telemetry(&self) -> impl IntoElement {
        let (fps, mean, p95) = self.stats.summary();
        let has_samples = !self.stats.samples.is_empty();
        let samples: Vec<f32> = self.stats.samples.iter().copied().collect();
        div()
            .h(px(130.))
            .flex_shrink_0()
            .px_6()
            .py_4()
            .bg(rgb(0xFFFFFF))
            .border_t_1()
            .border_color(rgb(LINE))
            .flex()
            .gap_8()
            .items_center()
            .child(
                div()
                    .w(px(174.))
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_size(px(13.))
                            .child("Measured on this machine"),
                    )
                    .child(label(match (self.running, has_samples) {
                        (true, true) => "Rolling 180 animation callbacks",
                        (false, true) => "Paused · last samples retained",
                        (true, false) => "Collecting timing samples…",
                        (false, false) => "No samples · resume to measure",
                    }))
                    .child(label("Timing includes scheduling.")),
            )
            .children(
                [
                    (
                        "Callback rate",
                        if has_samples {
                            format!("{fps:.0}")
                        } else {
                            "—".into()
                        },
                        "Hz",
                    ),
                    (
                        "Mean interval",
                        if has_samples {
                            format!("{mean:.1}")
                        } else {
                            "—".into()
                        },
                        "ms",
                    ),
                    (
                        "95th percentile",
                        if has_samples {
                            format!("{p95:.1}")
                        } else {
                            "—".into()
                        },
                        "ms",
                    ),
                ]
                .into_iter()
                .map(|(name, value, unit)| {
                    div()
                        .w(px(106.))
                        .flex_shrink_0()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(label(name))
                        .child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap_1()
                                .child(div().font_family("Menlo").text_size(px(25.)).child(value))
                                .child(label(unit)),
                        )
                }),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(label("Frame intervals"))
                            .child(label("16.7 ms reference")),
                    )
                    .child(
                        canvas(
                            |_, _, _| (),
                            move |bounds, _, window, _| {
                                let w = f32::from(bounds.size.width);
                                let h = f32::from(bounds.size.height);
                                let ox = f32::from(bounds.origin.x);
                                let oy = f32::from(bounds.origin.y);
                                let max_ms = samples.iter().copied().fold(33.4_f32, f32::max);
                                let reference = oy + h * (1. - 16.7 / max_ms);
                                window.paint_quad(fill(
                                    Bounds::new(point(px(ox), px(reference)), size(px(w), px(1.))),
                                    rgb(LINE),
                                ));
                                if samples.len() > 1 {
                                    let mut path = PathBuilder::stroke(px(1.5));
                                    for (i, ms) in samples.iter().enumerate() {
                                        let p = point(
                                            px(ox + i as f32 / 179. * w),
                                            px(oy + h * (1. - ms / max_ms)),
                                        );
                                        if i == 0 {
                                            path.move_to(p);
                                        } else {
                                            path.line_to(p);
                                        }
                                    }
                                    if let Ok(path) = path.build() {
                                        window.paint_path(path, rgb(ACCENT));
                                    }
                                }
                            },
                        )
                        .w_full()
                        .flex_1(),
                    ),
            )
    }

    fn command_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .top(px(68.))
            .right(px(24.))
            .w(px(340.))
            .p_5()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(LINE))
            .rounded_lg()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Keyboard controls"),
            )
            .children(
                [
                    ("Pause / resume", "Space"),
                    ("Switch scene", "S"),
                    ("Show / hide grid", "G"),
                    ("Raise / lower particle load", "] / ["),
                    ("Jump to record 50,000", "J"),
                    ("Select previous / next row", "Up / Down"),
                    ("First / last row", "Home / End"),
                    ("Show / hide this panel", "Cmd K"),
                    ("Close this panel", "Esc"),
                ]
                .into_iter()
                .map(|(name, key)| {
                    div()
                        .flex()
                        .justify_between()
                        .py_1()
                        .child(label(name))
                        .child(mono(key))
                }),
            )
            .child(button(
                "close-commands",
                "Close controls",
                false,
                cx,
                |s, _| s.commands = false,
            ))
    }
}

impl Render for Lab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.schedule(window, cx);
        div().id("gpui-lab").track_focus(&self.focus).key_context("Lab").relative()
            .on_action(cx.listener(Self::toggle_pause)).on_action(cx.listener(Self::toggle_scene))
            .on_action(cx.listener(Self::toggle_grid)).on_action(cx.listener(Self::raise))
            .on_action(cx.listener(Self::lower)).on_action(cx.listener(Self::toggle_commands))
            .on_action(cx.listener(Self::dismiss)).on_action(cx.listener(Self::first))
            .on_action(cx.listener(Self::last)).on_action(cx.listener(Self::middle))
            .on_action(cx.listener(Self::next)).on_action(cx.listener(Self::previous))
            .size_full().flex().flex_col().bg(rgb(PAPER)).text_color(rgb(INK))
            .font_family(".SystemUIFont").text_size(px(14.))
            .child(div().h(px(66.)).flex_shrink_0().px_6().flex().items_center().justify_between()
                .border_b_1().border_color(rgb(LINE))
                .child(div().flex().items_baseline().gap_4()
                    .child(div().text_size(px(22.)).font_weight(FontWeight::BOLD).child("GPUI Lab"))
                    .child(label("A native graphics & performance playground")))
                .child(div().flex().items_center().gap_4().child(label("Rust + Metal"))
                    .child(button("commands", "Keyboard controls   ⌘ K", false, cx, |s, _| s.commands = !s.commands))))
            .child(div().flex().flex_1().min_h(px(0.))
                .child(div().flex().flex_col().flex_1().min_w(px(0.))
                    .child(div().px_6().py_4().flex().justify_between().items_center()
                        .child(div().flex().flex_col().gap_1()
                            .child(div().text_size(px(20.)).font_weight(FontWeight::SEMIBOLD).child("Thousands of shapes. One native canvas."))
                            .child(label("GPU-painted circles, responsive controls, no web view.")))
                        .child(button("pause", if self.running { "Pause   Space" } else { "Resume   Space" }, true, cx, |s, _| s.pause())))
                    .child(self.field(cx))
                    .child(div().px_6().py_4().flex_shrink_0().flex().justify_between().items_center()
                        .child(div().flex().flex_col().gap_2().child(label("PARTICLE LOAD"))
                            .child(div().flex().gap_2()
                                .child(button("load-1k", "1,000", self.count == 1000, cx, |s, _| s.set_count(1000)))
                                .child(button("load-10k", "10,000", self.count == 10000, cx, |s, _| s.set_count(10000)))
                                .child(button("load-30k", "30,000", self.count == 30000, cx, |s, _| s.set_count(30000)))))
                        .child(div().flex().flex_col().gap_2().child(label("SCENE"))
                            .child(div().flex().gap_2()
                                .child(button("orbit", "Orbit", self.scene == Scene::Orbit, cx, |s, _| s.scene = Scene::Orbit))
                                .child(button("wave", "Wave", self.scene == Scene::Wave, cx, |s, _| s.scene = Scene::Wave))
                                .child(button("grid", "Grid", self.grid, cx, |s, _| s.grid = !s.grid)))))
                    .child(div().px_6().pb_4().text_size(px(12.)).text_color(rgb(MUTED))
                        .child("Positions are calculated in Rust; GPUI submits the shapes to Metal. Try 30,000, then scroll the list.")))
                .child(self.records(cx)))
            .child(self.telemetry())
            .child(div().px_6().h(px(34.)).flex_shrink_0().flex().items_center().justify_between()
                .border_t_1().border_color(rgb(LINE)).text_size(px(11.)).text_color(rgb(MUTED))
                .child("Actual GPUI 0.2.2  ·  Native macOS window  ·  Synthetic demonstration data")
                .child("Callback timing is not GPU execution time or a comparative benchmark."))
            .when(self.commands, |s| s.child(self.command_panel(cx)))
    }
}

struct ConsoleLogger;
impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}
static LOGGER: ConsoleLogger = ConsoleLogger;
fn main() {
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Warn);
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("space", TogglePause, Some("Lab")),
            KeyBinding::new("s", ToggleScene, Some("Lab")),
            KeyBinding::new("g", ToggleGrid, Some("Lab")),
            KeyBinding::new("]", RaiseLoad, Some("Lab")),
            KeyBinding::new("[", LowerLoad, Some("Lab")),
            KeyBinding::new("cmd-k", ToggleCommands, Some("Lab")),
            KeyBinding::new("escape", Dismiss, Some("Lab")),
            KeyBinding::new("j", JumpMiddle, Some("Lab")),
            KeyBinding::new("home", FirstRow, Some("Lab")),
            KeyBinding::new("end", LastRow, Some("Lab")),
            KeyBinding::new("down", NextRow, Some("Lab")),
            KeyBinding::new("up", PreviousRow, Some("Lab")),
            KeyBinding::new("cmd-q", Quit, None),
        ]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        let bounds = Bounds::centered(None, size(px(1280.), px(840.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(1080.), px(680.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI Lab".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Lab::new(window, cx)),
        )
        .unwrap();
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
