use druid::widget::{Align, Flex, Image, Label};
use druid::{Color, Widget, WidgetExt, ImageBuf, UnitPoint};
use crate::AppState;
use crate::utils::load_image;
use std::path::Path;

pub struct Inicio;

impl Inicio {
    pub fn build() -> impl Widget<AppState> {
        // Caminho da imagem a ser carregada
        let image_path = "src\\resources\\images\\texto_licitacao360.png";

        // Carregando a imagem
        let image_buf = load_image(Path::new(image_path)).unwrap_or_else(|e| {
            println!("Failed to load image: {}", e);
            ImageBuf::empty()
        });

        let image = Image::new(image_buf)
            .fill_mode(druid::widget::FillStrat::Contain)
            .fix_size(480.0, 360.0); // Reduzido em 20%

        let text_widget = Label::new("Desenvolvido por: CC (IM) Guilherme Kirschner de Siqueira Campos\nWhatsApp: (61) 98264-0077\nEmail: siqueira.campos@marinha.mil.br")
            .with_text_color(Color::WHITE)
            .padding((10.0, 10.0, 10.0, 10.0));

        let layout = Flex::column()
            .with_flex_child(
                Align::centered(image),
                1.0,
            )
            .with_child(
                Align::new(UnitPoint::BOTTOM_RIGHT, text_widget),
            )
            .background(Color::BLACK);

        layout
    }
}
