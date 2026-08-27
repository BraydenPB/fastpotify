//! Desktop entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod auth;
mod backend;
#[cfg(any(test, feature = "demo"))]
mod demo;
mod images;
mod model;
#[cfg(target_os = "linux")]
mod mpris;
#[cfg(not(target_os = "linux"))]
#[path = "mpris_stub.rs"]
mod mpris;
mod paths;
mod player;
mod settings;
mod theme;
mod ui;
mod util;

use clap::Parser;

/// A fast, native Spotify client.
#[derive(Debug, Parser)]
#[command(name = "fastpotify", version, about)]
struct Cli {
    /// Spotify Connect device name for this session.
    #[arg(long)]
    device_name: Option<String>,

    /// Log more from librespot and the Web API client.
    #[arg(short, long)]
    verbose: bool,

    /// Start with sample data and no Spotify connection (for screenshots).
    #[cfg(feature = "demo")]
    #[arg(long)]
    demo: bool,

    /// Page to open in demo mode, e.g. `home`, `playlist:pl1`, `artist:art0`.
    #[cfg(feature = "demo")]
    #[arg(long)]
    demo_page: Option<String>,

    /// Extra demo surfaces: a comma-separated list of `queue`, `devices`,
    /// `shortcuts`, `create`, `light`.
    #[cfg(feature = "demo")]
    #[arg(long)]
    demo_show: Option<String>,
}

fn main() -> eframe::Result<()> {
    let cli = Cli::parse();
    let default_filter = if cli.verbose {
        "info,librespot=info,fastpotify=debug"
    } else {
        "warn,fastpotify=info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_filter))
        .init();

    let dirs = paths::AppDirs::discover();
    if let Err(error) = dirs.ensure() {
        log::warn!("unable to create the application directories: {error}");
    }
    let mut settings = settings::Settings::load(&dirs.settings_file());
    if let Some(name) = cli.device_name {
        settings.device_name = name;
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Fastpotify")
            .with_app_id("fastpotify")
            .with_inner_size([1240.0, 800.0])
            .with_min_inner_size([760.0, 520.0])
            .with_icon(app_icon()),
        // A Wayland compositor stops sending frame callbacks to a hidden
        // window; waiting for vsync there would block the event loop.
        // Repaints are event-driven, so nothing spins.
        glow_options: eframe::egui_glow::GlowConfiguration {
            vsync: false,
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Fastpotify",
        options,
        Box::new(move |cc| {
            #[allow(unused_mut)]
            let mut app = app::App::new(&cc.egui_ctx, dirs, settings, app::AppOptions::default());
            #[cfg(feature = "demo")]
            if cli.demo {
                demo::populate(&mut app);
                demo::apply_flags(&mut app, cli.demo_page.as_deref(), cli.demo_show.as_deref());
            }
            Ok(Box::new(app))
        }),
    )
}

/// The window icon: a green disc with a play mark, drawn at start so no
/// binary image needs shipping.
fn app_icon() -> egui::IconData {
    const SIZE: usize = 128;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];
    let center = SIZE as f32 / 2.0;
    let radius = center - 2.0;
    let triangle = [
        (center - 12.0, center - 22.0),
        (center - 12.0, center + 22.0),
        (center + 26.0, center),
    ];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let distance = ((px - center).powi(2) + (py - center).powi(2)).sqrt();
            let coverage = (radius - distance + 0.5).clamp(0.0, 1.0);
            if coverage <= 0.0 {
                continue;
            }
            let inside = point_in_triangle((px, py), triangle);
            let (r, g, b) = if inside { (10, 20, 14) } else { (30, 215, 96) };
            let index = (y * SIZE + x) * 4;
            rgba[index] = r;
            rgba[index + 1] = g;
            rgba[index + 2] = b;
            rgba[index + 3] = (coverage * 255.0) as u8;
        }
    }
    egui::IconData {
        rgba,
        width: SIZE as u32,
        height: SIZE as u32,
    }
}

fn point_in_triangle(p: (f32, f32), t: [(f32, f32); 3]) -> bool {
    let sign = |a: (f32, f32), b: (f32, f32), c: (f32, f32)| {
        (a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1)
    };
    let d1 = sign(p, t[0], t[1]);
    let d2 = sign(p, t[1], t[2]);
    let d3 = sign(p, t[2], t[0]);
    let negative = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let positive = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(negative && positive)
}
