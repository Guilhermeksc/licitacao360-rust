// src/modules/atas/automacoes.rs

use druid::widget::Flex;
use druid::{Color, FontWeight, TextAlignment, Widget, WidgetExt};
use crate::AppState;
use crate::utils::utils_load::create_title_content;
// Estrutura para a "Controle de Contratos"
pub struct Automacoes;

impl Automacoes {
    pub fn build() -> impl Widget<AppState> {
        let title = create_title_content(
            "Automacoes",
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