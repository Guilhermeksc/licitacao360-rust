use druid::ImageBuf;
use druid::{Color, Widget, WidgetExt}; 
use crate::utils::load_image;
use druid::widget::{Flex, Align, Image, Label};
use crate::paths;
use crate::AppState;

pub struct Inicio;

impl Inicio {
    pub fn build() -> impl Widget<AppState> {
        println!("Iniciando build_widget");

        let image_buf = load_image(paths::texto_image_path().to_str().unwrap()).unwrap_or_else(|e| {
            println!("{}", e);
            ImageBuf::empty()
        });
        println!("Imagem carregada");

        let image_widget = Image::new(image_buf)
            .fill_mode(druid::widget::FillStrat::Contain)
            .fix_size(600.0, 450.0);

        let text_widget = Label::new("Desenvolvido por: CC (IM) Guilherme Kirschner de Siqueira Campos\nWhatsApp: (61) 98264-0077 Email: siqueira.campos@marinha.mil.br")
            .with_text_color(Color::WHITE)
            .align_right()
            .padding((10.0, 0.0, 10.0, 10.0));

        let layout = Flex::column()
            .with_child(Align::centered(image_widget))
            .with_child(text_widget)
            .background(Color::BLACK)
            .expand();

        println!("Layout criado");

        layout
    }
}
