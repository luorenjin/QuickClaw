mod app;
mod chat;
mod config;
mod wizard;

use app::QuickClawApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("QuickClaw - OpenClaw 桌面客户端")
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([600.0, 450.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "QuickClaw",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            setup_visuals(&cc.egui_ctx);
            Ok(Box::new(QuickClawApp::new(cc)))
        }),
    )
}

/// 设置字体 - 尝试加载 CJK 字体以支持中文显示
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试从系统字体目录加载 CJK 字体
    let cjk_font_paths = [
        // Linux
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/truetype/arphic/uming.ttc",
        "/usr/share/fonts/truetype/arphic/ukai.ttc",
        // macOS
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        "/Library/Fonts/Arial Unicode MS.ttf",
        // Windows
        "C:/Windows/Fonts/msyh.ttc",
        "C:/Windows/Fonts/simsun.ttc",
        "C:/Windows/Fonts/simhei.ttf",
    ];

    for path in &cjk_font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "cjk_font".to_owned(),
                egui::FontData::from_owned(font_data).into(),
            );
            // 将 CJK 字体添加为各字体族的备用字体（在默认字体后面）
            for family in [
                egui::FontFamily::Proportional,
                egui::FontFamily::Monospace,
            ] {
                fonts
                    .families
                    .entry(family)
                    .or_default()
                    .push("cjk_font".to_owned());
            }
            break;
        }
    }

    ctx.set_fonts(fonts);
}

/// 设置应用视觉主题（暗色主题）
fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // 自定义颜色
    visuals.panel_fill = egui::Color32::from_rgb(22, 22, 30);
    visuals.window_fill = egui::Color32::from_rgb(28, 28, 38);
    visuals.extreme_bg_color = egui::Color32::from_rgb(15, 15, 20);

    ctx.set_visuals(visuals);
    ctx.set_pixels_per_point(1.0);
}

/// 加载应用图标（使用内嵌的 SVG-like 像素图标）
fn load_icon() -> egui::IconData {
    // 生成一个简单的 32x32 蓝色蟹爪图标
    let size = 32u32;
    let mut pixels = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            // 简单的圆形图标
            let cx = size as f32 / 2.0;
            let cy = size as f32 / 2.0;
            let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();

            if dist < 14.0 {
                // 主蓝色圆形
                pixels[idx] = 0;       // R
                pixels[idx + 1] = 120; // G
                pixels[idx + 2] = 215; // B
                pixels[idx + 3] = 255; // A
            } else {
                pixels[idx + 3] = 0; // 透明
            }
        }
    }

    egui::IconData {
        rgba: pixels,
        width: size,
        height: size,
    }
}
