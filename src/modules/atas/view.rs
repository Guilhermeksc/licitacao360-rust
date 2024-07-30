// src/modules/atas/view.rs

use druid::widget::Flex;
use druid::{Color, FontWeight, TextAlignment, Widget, WidgetExt};
use crate::AppState;
use crate::utils::create_title_content;
// Estrutura para a "Controle de Contratos"
pub struct Atas;

impl Atas {
    pub fn build() -> impl Widget<AppState> {
        let title = create_title_content(
            "Dispensa Eletrônica",
            30.0,
            Color::WHITE,
            FontWeight::BOLD,
            TextAlignment::Start,
            10.0,
        );

        Flex::column()
            .with_child(title)
            .padding(10.0)
            .background(Color::BLACK)
    }
}