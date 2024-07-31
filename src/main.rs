mod controller;
mod modules;
mod utils;
mod paths;
mod data_wrapper;

use druid::widget::{Container, Flex, Label, ViewSwitcher};
use druid::{AppLauncher, Data, Lens, WindowDesc, Color, Widget, WidgetExt, Env, Size};
use paths::Paths;
use controller::menu_superior::{build_menu, Delegate};
use controller::menu_lateral::create_menu_buttons;
use modules::inicio::controller::{InicioController, Inicio};
use modules::planejamento::controller::PlanejamentoController;
use modules::planejamento::view::Planejamento;
use modules::planejamento::data::PlanejamentoData;
use modules::dispensa_eletronica::controller::DispensaEletronicaController;
use modules::dispensa_eletronica::view::DispensaEletronica;
use modules::dispensa_eletronica::data::DispensaEletronicaData;
use modules::atas::controller::AtasController;
use modules::atas::view::Atas;
use modules::atas::data::AtasData;
use modules::contratos::controller::ContratosController;
use modules::contratos::view::Contratos;
use modules::contratos::data::ContratosData;
use modules::matriz_riscos::controller::MatrizRiscosController;
use modules::matriz_riscos::view::MatrizRiscos;
use modules::matriz_riscos::data::MatrizRiscosData;
use modules::automacoes::controller::AutomacoesController;
use modules::automacoes::view::Automacoes;
use modules::automacoes::data::AutomacoesData;

use data_wrapper::DataFrameWrapper;

use gtk::prelude::*;
use gtk::Application;
use std::sync::Arc;

#[derive(Clone, Data)]
pub enum ControllerEnum {
    Inicio(Arc<InicioController>),
    Planejamento(Arc<PlanejamentoController>),
    DispensaEletronica(Arc<DispensaEletronicaController>),
    Atas(Arc<AtasController>),
    Contratos(Arc<ContratosController>),
    MatrizRiscos(Arc<MatrizRiscosController>),
    Automacoes(Arc<AutomacoesController>),
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub paths: Paths,
    pub current_view: String,
    pub controllers: ControllerEnum,
    pub planejamento_data: DataFrameWrapper,
    pub contratos_data: DataFrameWrapper,
    pub atas_data: DataFrameWrapper,
    pub dispensa_eletronica_data: DataFrameWrapper,
    pub matriz_riscos_data: DataFrameWrapper,
    pub automacoes_data: DataFrameWrapper,
}

impl Default for AppState {
    fn default() -> Self {
        let paths = Paths::new(std::env::current_dir().expect("Failed to get current directory"));
        paths.create_database_dirs();
        paths.create_parquet_files();
        Self {
            paths,
            current_view: "Início".into(),
            controllers: ControllerEnum::Inicio(Arc::new(InicioController::default())),
            planejamento_data: DataFrameWrapper(None),
            contratos_data: DataFrameWrapper(None),
            atas_data: DataFrameWrapper(None),
            dispensa_eletronica_data: DataFrameWrapper(None),
            matriz_riscos_data: DataFrameWrapper(None),
            automacoes_data: DataFrameWrapper(None),
        }
    }
}

const IMPACT_PHRASE: &str = "\"Simplicidade é melhor do que complexidade. Complexidade é melhor do que complicação.\" (Tim Peters)";

fn main() {
    let initial_state = AppState::default();

    let main_window = WindowDesc::new(build_root_widget(&initial_state.paths))
        .title("Licitacao360")
        .menu(|id, state, env| build_menu(id, state, env))
        .window_size((1400.0, 800.0))
        .with_min_size(Size::new(800.0, 700.0));

    // Inicialize a aplicação GTK
    let gtk_app = Application::new(Some("com.example.gtk-planejamento"), Default::default());
    gtk_app.connect_activate(|app| {
        Planejamento::build(app);
    });
    gtk_app.run();

    // Inicialize a aplicação Druid
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget(paths: &Paths) -> impl Widget<AppState> {
    Flex::column()
        .with_child(create_impact_phrase())
        .with_flex_child(
            Flex::row()
                .with_child(create_menu_buttons(paths))
                .with_flex_child(create_content_area(), 1.0),
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

fn create_content_area() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env: &Env| data.current_view.clone(),
        |selector, data, _env| {
            Box::new(build_widget_for_selector(selector, data.clone()))
        },
    )
}

fn build_widget_for_selector(selector: &str, mut data: AppState) -> impl Widget<AppState> {
    let content: Box<dyn Widget<AppState>> = match selector {
        "Início" => Box::new(Inicio::build()),
        "Planejamento" => {
            if data.planejamento_data.0.is_none() {
                let df = PlanejamentoData::load_or_create(&data.paths);
                data.planejamento_data.0 = Some(df);
            }
            Box::new(Flex::column()) // Ajuste aqui para retornar um widget vazio por enquanto
        },
        "Atas" => {
            if data.atas_data.0.is_none() {
                let df = AtasData::load_or_create(&data.paths);
                data.atas_data.0 = Some(df);
            }
            Box::new(Atas::build())
        },
        "Contratos" => {
            if data.contratos_data.0.is_none() {
                let df = ContratosData::load_or_create(&data.paths);
                data.contratos_data.0 = Some(df);
            }
            Box::new(Contratos::build())
        },
        "Dispensa Eletrônica" => {
            if data.dispensa_eletronica_data.0.is_none() {
                let df = DispensaEletronicaData::load_or_create(&data.paths);
                data.dispensa_eletronica_data.0 = Some(df);
            }
            Box::new(DispensaEletronica::build())
        },
        "Matriz de Riscos" => {
            if data.matriz_riscos_data.0.is_none() {
                let df = MatrizRiscosData::load_or_create(&data.paths);
                data.matriz_riscos_data.0 = Some(df);
            }
            Box::new(MatrizRiscos::build())
        },
        "Automações" => {
            if data.automacoes_data.0.is_none() {
                let df = AutomacoesData::load_or_create(&data.paths);
                data.automacoes_data.0 = Some(df);
            }
            Box::new(Automacoes::build())
        },
        _ => Box::new(Inicio::build()),
    };

    Container::new(content)
        .padding(10.0)
        .background(Color::BLACK)
        .expand()
        .boxed()
}
