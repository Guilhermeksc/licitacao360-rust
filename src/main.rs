use druid::widget::Container;
use druid::widget::{Flex, Label, ViewSwitcher};
use druid::{AppLauncher, Data, Lens, WindowDesc, Color, Widget, WidgetExt, Env, Size};

mod controller;
mod modules;
mod paths;
mod utils;

use paths::Paths;
use controller::menu_superior::{build_menu, Delegate};
use controller::menu_lateral::create_menu_buttons;
use modules::inicio::view::Inicio;
use modules::planejamento::view::Planejamento;
use modules::dispensa_eletronica::view::DispensaEletronica;
use modules::atas::view::Atas;
use modules::contratos::view::Contratos;
use modules::matriz_riscos::view::MatrizRiscos;
use modules::automacoes::view::Automacoes;

#[derive(Clone, Data, Lens)]
struct AppState {
    paths: Paths,
    current_view: String,
}

const IMPACT_PHRASE: &str = "\"Simplicidade é melhor do que complexidade. Complexidade é melhor do que complicação.\" (Tim Peters)";

fn main() {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let paths = Paths::new(base_path.clone());

    let initial_state = AppState {
        paths: paths.clone(),
        current_view: "Início".into(),
    };

    let main_window = WindowDesc::new(build_root_controller(paths))
        .title("Licitacao360")
        .menu(|id, state, env| build_menu(id, state, env))
        .window_size((1400.0, 800.0)) // Define o tamanho inicial da janela
        .with_min_size(Size::new(800.0, 700.0)); // Define o tamanho mínimo da janela

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_controller(paths: Paths) -> impl Widget<AppState> {
    let impact_phrase = create_impact_phrase();
    let menu_buttons = create_menu_buttons(&paths);

    let content_area = ViewSwitcher::new(
        |data: &AppState, _env: &Env| data.current_view.clone(),
        |selector, _data, _env| {
            let content: Box<dyn Widget<AppState>> = match selector.as_str() {
                "Início" => Box::new(Inicio::build()),
                "Planejamento" => Box::new(Planejamento::build()),
                "Atas" => Box::new(Atas::build()),
                "Contratos" => Box::new(Contratos::build()),
                "Dispensa Eletrônica" => Box::new(DispensaEletronica::build()),
                "Matriz de Riscos" => Box::new(MatrizRiscos::build()),
                "Selenium" => Box::new(Automacoes::build()),
                _ => Box::new(Inicio::build()),
            };

            Container::new(content)
                .padding(10.0)
                .background(Color::BLACK)
                .expand()
                .boxed()
        },
    );

    Flex::column()
        .with_child(impact_phrase)
        .with_flex_child(
            Flex::row()
                .with_child(menu_buttons)
                .with_flex_child(content_area, 1.0),
            1.0,
        )
}

fn create_impact_phrase() -> impl Widget<AppState> {
    Label::new(IMPACT_PHRASE)
        .with_text_color(Color::WHITE)
        .with_text_size(16.0)
        .padding((1.0, 2.0))
        .align_horizontal(druid::UnitPoint::CENTER)
}
