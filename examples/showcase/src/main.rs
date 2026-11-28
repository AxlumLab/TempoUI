use tempo_ui::navigation;

use std::{
    cell::Cell,
    error::Error,
    fs::File,
    path::Path,
    rc::Rc,
    time::{Duration, Instant},
};

use slint::{ComponentHandle, LogicalSize, Model, Timer, VecModel};

slint::include_modules!();

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
struct Options {
    compact: bool,
    dark: bool,
    wide: bool,
    layout: bool,
    lists: bool,
    motion_smoke: bool,
    screenshot: Option<String>,
    smoke: bool,
}

fn options() -> Result<Options> {
    let mut options = Options::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--compact" => options.compact = true,
            "--dark" => options.dark = true,
            "--wide" => options.wide = true,
            "--layout" => {
                options.layout = true;
            }
            "--lists" => {
                options.lists = true;
            }
            "--motion-smoke" => {
                options.motion_smoke = true;
            }
            "--screenshot" => options.screenshot = Some(args.next().ok_or("missing PNG path")?),
            "--smoke" => options.smoke = true,
            "--help" | "-h" => {
                println!(
                    "tempo-ui-showcase [--compact] [--dark] [--layout|--lists] [--wide] [--screenshot PATH] [--smoke|--motion-smoke]"
                );
                std::process::exit(0);
            }
            _ => return Err(format!("unknown option: {arg}").into()),
        }
    }
    Ok(options)
}

fn save_screenshot(window: &slint::Window, path: &Path) -> Result<()> {
    let snapshot = window.take_snapshot()?;
    let mut encoder = png::Encoder::new(File::create(path)?, snapshot.width(), snapshot.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()?
        .write_image_data(snapshot.as_bytes())?;
    Ok(())
}

fn click_in(window: &slint::Window, x: f32, y: f32) {
    use slint::platform::{PointerEventButton, WindowEvent};
    let position = slint::LogicalPosition::new(x, y);
    let button = PointerEventButton::Left;
    window.dispatch_event(WindowEvent::PointerMoved { position });
    window.dispatch_event(WindowEvent::PointerPressed { position, button });
    window.dispatch_event(WindowEvent::PointerReleased { position, button });
}

fn drag(window: &slint::Window, x: f32, y: f32, dy: f32) {
    use slint::platform::{PointerEventButton, WindowEvent};
    let start = slint::LogicalPosition::new(x, y);
    let button = PointerEventButton::Left;
    window.dispatch_event(WindowEvent::PointerMoved { position: start });
    window.dispatch_event(WindowEvent::PointerPressed {
        position: start,
        button,
    });
    for step in 1..=8 {
        let position = slint::LogicalPosition::new(x, y + dy * step as f32 / 8.0);
        window.dispatch_event(WindowEvent::PointerMoved { position });
    }
    let end = slint::LogicalPosition::new(x, y + dy);
    window.dispatch_event(WindowEvent::PointerReleased {
        position: end,
        button,
    });
}

fn sync_route(ui: &TempoCatalogWindow, stack: &navigation::RouteStacks<i32>, tab: usize) {
    ui.set_detail(stack.top(tab).copied().unwrap_or(-1));
    ui.set_route_depth(stack.depth(tab) as i32);
}

fn move_index<T>(items: &mut Vec<T>, from: usize, to: usize) -> bool {
    if from == to || from >= items.len() || to >= items.len() {
        return false;
    }
    let value = items.remove(from);
    items.insert(to, value);
    true
}

fn wire_catalog(ui: &TempoCatalogWindow) {
    use std::cell::RefCell;
    let queue = Rc::new(VecModel::from(
        [
            "Earthly Light",
            "Prelude",
            "Sea Glass",
            "City Static",
            "Blue Hour",
            "Paper Planes",
        ]
        .into_iter()
        .map(|name| TempoReorderItem {
            label: name.into(),
            subtitle: "Queue sample".into(),
        })
        .collect::<Vec<_>>(),
    ));
    ui.set_queue_samples(queue.clone().into());
    ui.on_reorder_request(move |from, to| {
        let Ok(from) = usize::try_from(from) else {
            return;
        };
        let Ok(to) = usize::try_from(to) else { return };
        let mut items: Vec<_> = queue.iter().collect();
        if move_index(&mut items, from, to) {
            queue.set_vec(items);
        }
    });
    let stack = Rc::new(RefCell::new(navigation::RouteStacks::default()));
    let weak = ui.as_weak();
    let routes = stack.clone();
    ui.on_push_detail(move |id, replace| {
        if let Some(ui) = weak.upgrade() {
            let tab = ui.get_tab() as usize;
            ui.set_route_direction(1);
            routes.borrow_mut().open(tab, id, replace);
            sync_route(&ui, &routes.borrow(), tab);
        }
    });
    let weak = ui.as_weak();
    let routes = stack.clone();
    ui.on_pop_detail(move || {
        if let Some(ui) = weak.upgrade() {
            let tab = ui.get_tab() as usize;
            ui.set_route_direction(-1);
            routes.borrow_mut().back(tab);
            sync_route(&ui, &routes.borrow(), tab);
        }
    });
    let weak = ui.as_weak();
    ui.on_close_route(move || {
        if let Some(ui) = weak.upgrade() {
            let tab = ui.get_tab() as usize;
            ui.set_route_direction(-1);
            stack.borrow_mut().close(tab);
            sync_route(&ui, &stack.borrow(), tab);
        }
    });
}

fn press(window: &slint::Window, key: slint::platform::Key) {
    use slint::platform::WindowEvent;
    window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

fn catalog_smoke_start(
    ui: &TempoCatalogWindow,
    compact: bool,
    dark: bool,
    wide: bool,
) -> Result<()> {
    use slint::platform::Key;
    if ui.get_compact() != compact || ui.get_dark() != dark {
        return Err("catalog viewport or theme did not apply".into());
    }
    if ui.get_group_marker_width() < 40.0
        || ui.get_icon_marker_width() < 40.0
        || !ui.invoke_refresh_language_is_chinese()
    {
        return Err("tab marker or localized refresh strings did not resolve".into());
    }
    let original_origin = ui.get_heading_origin();
    ui.set_extra_insets(TempoInsets {
        top: 21.0,
        left: 10.0,
        right: 9.0,
        bottom: 18.0,
    });
    let safe_origin = ui.get_heading_origin();
    if (safe_origin.y - original_origin.y - 21.0).abs() > 1.0 {
        return Err("heading was not padded by the injected safe area".into());
    }
    ui.set_extra_insets(TempoInsets::default());
    let group = ui.get_group_origin();
    click_in(ui.window(), group.x + 131.0, group.y + 23.0);
    if ui.get_group_index() != 1 {
        return Err("button-group click did not select the next tab".into());
    }
    let radio = ui.get_radio_point();
    click_in(ui.window(), radio.x, radio.y);
    if !ui.get_radio_focused() || ui.get_radio_focus_ring() {
        return Err("pointer activation of the radio added a focus ring or lost focus".into());
    }
    press(ui.window(), Key::Space);
    if !ui.get_radio_focus_ring() {
        return Err("keyboard activation of the focused radio did not show focus".into());
    }
    click_in(ui.window(), radio.x, radio.y);
    if ui.get_radio_focus_ring() {
        return Err("pointer activation did not clear the keyboard focus ring".into());
    }
    let field = ui.get_field_center();
    click_in(ui.window(), field.x, field.y);
    use slint::platform::WindowEvent;
    ui.window()
        .dispatch_event(WindowEvent::KeyPressed { text: "M".into() });
    ui.window()
        .dispatch_event(WindowEvent::KeyReleased { text: "M".into() });
    press(ui.window(), Key::Return);
    if ui.get_field_value() != "M" || ui.get_submitted() != "M" {
        return Err("text input did not edit and submit through the keyboard".into());
    }
    ui.invoke_open_combo();
    if !ui.get_combo_open() || ui.get_combo_is_sheet() != compact {
        return Err("choice popup did not use the expected desktop or compact presentation".into());
    }
    press(ui.window(), Key::DownArrow);
    press(ui.window(), Key::Return);
    if ui.get_selected_choice() != -1 || !ui.get_combo_open() {
        return Err("keyboard accepted a disabled item".into());
    }
    press(ui.window(), Key::DownArrow);
    press(ui.window(), Key::Return);
    if ui.get_selected_choice() != 2 || ui.get_combo_open() {
        return Err(format!(
            "keyboard choice did not select the enabled item: {}",
            ui.get_selected_choice()
        )
        .into());
    }
    ui.invoke_show_dialog();
    if !ui.get_dialog_open() {
        return Err("dialog did not open".into());
    }
    press(ui.window(), Key::Escape);
    if ui.get_dialog_open() {
        return Err("Escape did not dismiss dialog".into());
    }
    ui.set_tab(1);
    if ui.get_wide_split() == compact || ui.get_three_pane() != wide {
        return Err("adaptive split did not match the viewport".into());
    }
    ui.invoke_push_detail(1, true);
    ui.invoke_push_detail(4, false);
    if ui.get_detail() != 4 || ui.get_route_depth() != 2 {
        return Err("nested route was not pushed".into());
    }
    ui.set_tab(0);
    ui.set_tab(1);
    ui.invoke_pop_detail();
    if ui.get_detail() != 1 || ui.get_route_depth() != 1 {
        return Err("back did not restore the previous route".into());
    }
    ui.invoke_close_route();
    if ui.get_route_depth() != 0 {
        return Err("close did not empty the route stack".into());
    }
    ui.set_tab(2);
    let refresh = ui.get_refresh_origin();
    drag(ui.window(), refresh.x + 100.0, refresh.y + 30.0, 155.0);
    ui.invoke_request_more();
    if ui.get_refresh_count() != 1 || ui.get_load_count() != 0 || ui.get_refresh_idle() {
        return Err("refresh did not reject overlapping load requests".into());
    }
    ui.invoke_preview_toast();
    if !ui.get_toast_visible() {
        return Err("toast did not open".into());
    }
    Ok(())
}

fn run_catalog(options: Options) -> Result<()> {
    let ui = TempoCatalogWindow::new()?;
    ui.set_dark(options.dark);
    wire_catalog(&ui);
    let size = if options.compact {
        LogicalSize::new(390.0, 780.0)
    } else if options.wide {
        LogicalSize::new(1700.0, 850.0)
    } else {
        LogicalSize::new(1180.0, 820.0)
    };
    ui.window().set_size(size);
    if options.lists {
        ui.set_tab(3);
    }
    if options.layout {
        ui.set_tab(1);
        ui.invoke_push_detail(1, true);
    }
    if options.motion_smoke {
        return run_motion_smoke(&ui, options.screenshot);
    }
    if options.screenshot.is_none() && !options.smoke {
        ui.run()?;
        return Ok(());
    }
    let result = Rc::new(std::cell::RefCell::new(None));
    let outcome = result.clone();
    let weak = ui.as_weak();
    Timer::single_shot(Duration::from_millis(500), move || {
        let initial = (|| -> Result<()> {
            let ui = weak.upgrade().ok_or("catalog closed before validation")?;
            if let Some(path) = &options.screenshot {
                save_screenshot(ui.window(), Path::new(path))?;
            }
            if options.smoke {
                ui.set_tab(0);
                catalog_smoke_start(&ui, options.compact, options.dark, options.wide)?;
            }
            Ok(())
        })();
        if initial.is_err() || !options.smoke {
            *outcome.borrow_mut() = Some(initial);
            let _ = slint::quit_event_loop();
            return;
        }

        Timer::single_shot(Duration::from_millis(440), move || {
            let check = (|| -> Result<()> {
                let ui = weak
                    .upgrade()
                    .ok_or("catalog closed during refresh validation")?;
                if ui.get_toast_visible() || !ui.get_refresh_idle() {
                    return Err("toast or refresh timer did not settle".into());
                }
                ui.invoke_request_more();
                if ui.get_load_count() != 1 {
                    return Err("load callback did not run".into());
                }
                Ok(())
            })();
            if check.is_err() {
                *outcome.borrow_mut() = Some(check);
                let _ = slint::quit_event_loop();
                return;
            }
            Timer::single_shot(Duration::from_millis(400), move || {
                let check = (|| -> Result<()> {
                    let ui = weak.upgrade().ok_or("catalog closed during pagination")?;
                    ui.invoke_request_more();
                    if !ui.get_refresh_exhausted() || ui.get_load_count() != 1 {
                        return Err("pagination restarted after no-more result".into());
                    }
                    if !ui.get_refresh_footer_visible() {
                        return Err("pagination end notice never appeared".into());
                    }
                    Ok(())
                })();
                if check.is_err() {
                    *outcome.borrow_mut() = Some(check);
                    let _ = slint::quit_event_loop();
                    return;
                }
                Timer::single_shot(Duration::from_millis(1800), move || {
                    let check = (|| -> Result<()> {
                        let ui = weak
                            .upgrade()
                            .ok_or("catalog closed while the end notice was visible")?;
                        ui.invoke_request_more();
                        if ui.get_refresh_footer_visible()
                            || !ui.get_refresh_exhausted()
                            || ui.get_load_count() != 1
                        {
                            return Err(
                                "pagination end notice did not hide or pagination resumed".into()
                            );
                        }
                        ui.set_tab(3);
                        let grip = ui.get_reorder_origin();
                        drag(
                            ui.window(),
                            grip.x + ui.get_reorder_width() - 12.0,
                            grip.y + 24.0,
                            122.0,
                        );
                        if ui.get_reorder_count() != 1 || ui.get_first_queue_entry() != "Prelude" {
                            return Err("queue grip did not reorder the model".into());
                        }
                        ui.invoke_open_actions();
                        press(ui.window(), slint::platform::Key::DownArrow);
                        press(ui.window(), slint::platform::Key::Return);
                        if ui.get_chosen_menu() != -1 || !ui.get_menu_open() {
                            return Err("disabled menu action was accepted".into());
                        }
                        press(ui.window(), slint::platform::Key::DownArrow);
                        press(ui.window(), slint::platform::Key::Return);
                        if ui.get_chosen_menu() != 2 || ui.get_menu_open() {
                            return Err("keyboard did not select the enabled menu entry".into());
                        }
                        let initial = ui.get_instantiated_rows();
                        ui.invoke_reveal_large_row();
                        let after = ui.get_instantiated_rows();
                        if initial >= 300
                            || after == 0
                            || after >= 500
                            || ui.get_large_scroll_y() >= -100_000.0
                        {
                            return Err(format!("large list not virtualized or scroll failed: {initial} / {after}, y={}", ui.get_large_scroll_y()).into());
                        }
                        println!(
                            "Tempo UI catalog smoke check passed (virtual list delegates: {initial} → {after})"
                        );
                        Ok(())
                    })();
                    *outcome.borrow_mut() = Some(check);
                    let _ = slint::quit_event_loop();
                });
            });
        });
    });
    ui.run()?;
    return result
        .borrow_mut()
        .take()
        .ok_or("catalog closed before results were ready")?;
}

fn run_motion_smoke(ui: &TempoCatalogWindow, screenshot: Option<String>) -> Result<()> {
    use slint::platform::Key;
    let outcome = Rc::new(std::cell::RefCell::new(None));
    let result = outcome.clone();
    let weak = ui.as_weak();
    let probe = Timer::default();
    let stage = Cell::new(0);
    let started = Cell::new(Instant::now());
    let sampled_dialog = Cell::new(false);
    let sampled_toast = Cell::new(false);
    let sampled_exit = Cell::new(false);
    let sampled_route = Cell::new(false);
    probe.start(slint::TimerMode::Repeated, Duration::from_millis(28), move || {
        let elapsed = started.get().elapsed();
        let mid = |value: f32| value > 0.02 && value < 0.98;
        let Ok(ui) = weak.upgrade().ok_or("motion preview closed") else { return };
        let check: Result<bool> = (|| {
            match stage.get() {
                0 if elapsed >= Duration::from_millis(320) => {
                    ui.invoke_preview_toast();
                    ui.invoke_show_dialog();
                    stage.set(1); started.set(Instant::now());
                }
                1 => {
                    sampled_dialog.set(sampled_dialog.get() || mid(ui.get_dialog_progress()));
                    sampled_toast.set(sampled_toast.get() || mid(ui.get_toast_progress()));
                    if elapsed >= Duration::from_millis(275) {
                        if !sampled_dialog.get() || !sampled_toast.get() || ui.get_dialog_progress() < 0.98 {
                            return Err(format!("no intermediate entry frames: dialog {}, toast {}, progress {}", sampled_dialog.get(), sampled_toast.get(), ui.get_dialog_progress()).into());
                        }
                        let window = ui.window();
                        let size = window.size().to_logical(window.scale_factor());
                        click_in(window, size.width / 2.0, size.height / 2.0);
                        if !ui.get_dialog_open() {
                            return Err("window center hit the backdrop instead of the dialog".into());
                        }
                        if let Some(path) = &screenshot {
                            save_screenshot(window, Path::new(path))?;
                        }
                        press(window, Key::Escape);
                        stage.set(2); started.set(Instant::now());
                    }
                }
                2 => {
                    sampled_exit.set(sampled_exit.get() || mid(ui.get_dialog_progress()));
                    if elapsed >= Duration::from_millis(245) {
                        if !sampled_exit.get() || ui.get_dialog_presented() {
                            return Err("dialog did not animate out and release its popup".into());
                        }
                        ui.set_tab(1);
                        ui.invoke_push_detail(1, true);
                        stage.set(3); started.set(Instant::now());
                    }
                }
                3 => {
                    sampled_route.set(sampled_route.get() || mid(ui.get_detail_progress()));
                    if elapsed >= Duration::from_millis(300) {
                        if !sampled_route.get() || ui.get_detail_progress() < 0.98 {
                            return Err("route transition had no intermediate frames or never settled".into());
                        }
                        ui.set_route_direction(0);
                        ui.set_detail(2);
                        stage.set(4); started.set(Instant::now());
                    }
                }
                4 => {
                    if mid(ui.get_detail_progress()) {
                        ui.set_reduced_motion(true);
                        ui.invoke_pop_detail();
                        ui.invoke_show_dialog();
                        if ui.get_detail_progress() < 0.999 || ui.get_dialog_progress() < 0.999 {
                            return Err("reduced motion did not bypass animation".into());
                        }
                        return Ok(true);
                    }
                    if elapsed >= Duration::from_millis(250) {
                        return Err("sibling destination did not animate".into());
                    }
                }
                _ => {}
            }
            Ok(false)
        })();
        match check {
            Ok(false) => {}
            other => {
                *result.borrow_mut() = Some(other.map(|_| ()));
                let _ = slint::quit_event_loop();
            }
        }
    });
    ui.run()?;
    outcome
        .borrow_mut()
        .take()
        .ok_or("motion checks did not finish")??;
    println!("Tempo UI motion smoke check passed");
    Ok(())
}

fn main() -> Result<()> {
    tempo_ui::fonts::configure_cjk_fallbacks()?;
    run_catalog(options()?)
}
