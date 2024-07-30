
// src/controller/menu_lateral.rs

use druid::{
    widget::{Flex, Label, Painter, Container, Image},
    Widget, WidgetExt, Color, FontDescriptor, FontFamily, UnitPoint, RenderContext, Env,
};
use druid::ImageBuf;
use crate::AppState;
use crate::utils::load_image;
use crate::paths::Paths;

pub const BUTTON_HEIGHT: f64 = 35.0;
pub const BUTTON_WIDTH: f64 = 165.0;
pub const BUTTON_FONT_SIZE: f64 = 16.0;

pub fn create_menu_buttons(paths: &Paths) -> Flex<AppState> {
    let menu_buttons = Flex::column()
        .with_child(create_custom_button("Início", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Planejamento", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Atas", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Contratos", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Dispensa Eletrônica", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Matriz de Riscos", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_child(create_custom_button("Selenium", BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_FONT_SIZE))
        .with_flex_spacer(1.0);

    let image_path = paths.menu_image_path();
    println!("Loading image from: {}", image_path.display());

    let image_logo = load_image(&image_path).unwrap_or_else(|e| {
        println!("Failed to load image: {}", e);
        ImageBuf::empty()
    });

    let image_widget = Image::new(image_logo)
        .fill_mode(druid::widget::FillStrat::Contain)
        .fix_size(165.0, 165.0);

    menu_buttons.with_child(image_widget)
}

pub fn create_custom_button(label: &str, button_width: f64, button_height: f64, font_size: f64) -> impl Widget<AppState> {
    let label_owned = label.to_string();
    let custom_label = Label::new(label_owned.clone())
        .with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_size))
        .with_text_color(Color::WHITE)
        .align_vertical(UnitPoint::CENTER);

    let painter_label = label_owned.clone();
    let painter = Painter::new(move |ctx, data: &AppState, _env: &Env| {
        let _ = _env;

        let is_hovered = ctx.is_hot();
        let is_active = ctx.is_active();
        let is_selected = data.current_view == painter_label;

        let background_color = if is_selected {
            Color::BLACK
        } else if is_active {
            Color::rgb8(0x30, 0x30, 0x30)
        } else if is_hovered {
            Color::rgb8(0x40, 0x40, 0x40)
        } else {
            Color::rgb8(0x20, 0x20, 0x20)
        };

        let border_color = Color::BLACK;
        let border_width = 0.5;

        let size = ctx.size();
        let rect = size.to_rect();

        ctx.fill(rect, &background_color);
        ctx.stroke(rect.with_size((size.width, border_width)), &border_color, border_width);
        ctx.stroke(rect.with_origin((0.0, size.height - border_width)).with_size((size.width, border_width)), &border_color, border_width);
    });

    Container::new(custom_label)
        .background(painter)
        .fix_size(button_width, button_height)
        .on_click(move |_, data: &mut AppState, _| {
            data.current_view = label_owned.clone();
        })
}
