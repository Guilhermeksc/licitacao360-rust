mod controller;
mod modules;
mod utils;
mod database;
mod data_wrapper;

use druid::widget::Container;
use druid::widget::{Flex, Label, ViewSwitcher};
use druid::{AppLauncher, Data, Lens, WindowDesc, Color, Widget, WidgetExt, Env, Size};
use database::Paths;
use controller::menu_superior::{build_menu, Delegate};
use controller::menu_lateral::create_menu_buttons;
use modules::inicio::controller::{InicioController, Inicio};
use modules::planejamento::controller::PlanejamentoController;
use modules::dispensa_eletronica::controller::DispensaEletronicaController;
use modules::atas::controller::AtasController;
use modules::contratos::controller::ContratosController;
use modules::matriz_riscos::controller::MatrizRiscosController;
use modules::automacoes::controller::AutomacoesController;
use modules::planejamento::view::Planejamento;
use modules::dispensa_eletronica::view::DispensaEletronica;
use modules::atas::view::Atas;
use modules::contratos::view::Contratos;
use modules::matriz_riscos::view::MatrizRiscos;
use modules::automacoes::view::Automacoes;
use data_wrapper::DataFrameWrapper;

use std::sync::Arc;
use polars::lazy::frame::LazyFrame;

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
        }
    }
}

const IMPACT_PHRASE: &str = "\"Simplicidade é melhor do que complexidade. Complexidade é melhor do que complicação.\" (Tim Peters)";

fn main() {
    let initial_state = AppState::default();

    let main_window = WindowDesc::new(build_root_controller(&initial_state.paths))
        .title("Licitacao360")
        .menu(|id, state, env| build_menu(id, state, env))
        .window_size((1400.0, 800.0))
        .with_min_size(Size::new(800.0, 700.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_controller(paths: &Paths) -> impl Widget<AppState> {
    let impact_phrase = create_impact_phrase();
    let menu_buttons = create_menu_buttons(paths);

    let content_area = ViewSwitcher::new(
        |data: &AppState, _env: &Env| data.current_view.clone(),
        |selector, data, _env| {
            let mut data = data.clone();
            let content: Box<dyn Widget<AppState>> = match selector.as_str() {
                "Início" => Box::new(Inicio::build()),
                "Planejamento" => {
                    if data.planejamento_data.0.is_none() {
                        let parquet_path = &data.paths.parquet_files[1].0;
                        let df = LazyFrame::scan_parquet(parquet_path.to_str().unwrap(), Default::default())
                            .expect("Failed to read planejamento.parquet")
                            .collect()
                            .expect("Failed to collect LazyFrame to DataFrame");
                        data.planejamento_data.0 = Some(df);
                    }
                    Box::new(Planejamento::build())
                },
                "Atas" => {
                    if data.atas_data.0.is_none() {
                        let parquet_path = &data.paths.parquet_files[2].0;
                        let df = LazyFrame::scan_parquet(parquet_path.to_str().unwrap(), Default::default())
                            .expect("Failed to read atas.parquet")
                            .collect()
                            .expect("Failed to collect LazyFrame to DataFrame");
                        data.atas_data.0 = Some(df);
                    }
                    Box::new(Atas::build())
                },
                "Contratos" => {
                    if data.contratos_data.0.is_none() {
                        let parquet_path = &data.paths.parquet_files[0].0;
                        let df = LazyFrame::scan_parquet(parquet_path.to_str().unwrap(), Default::default())
                            .expect("Failed to read contratos.parquet")
                            .collect()
                            .expect("Failed to collect LazyFrame to DataFrame");
                        data.contratos_data.0 = Some(df);
                    }
                    Box::new(Contratos::build())
                },
                "Dispensa Eletrônica" => {
                    if data.dispensa_eletronica_data.0.is_none() {
                        let parquet_path = &data.paths.parquet_files[3].0;
                        let df = LazyFrame::scan_parquet(parquet_path.to_str().unwrap(), Default::default())
                            .expect("Failed to read dispensa_eletronica.parquet")
                            .collect()
                            .expect("Failed to collect LazyFrame to DataFrame");
                        data.dispensa_eletronica_data.0 = Some(df);
                    }
                    Box::new(DispensaEletronica::build())
                },
                "Matriz de Riscos" => {
                    if data.matriz_riscos_data.0.is_none() {
                        let parquet_path = &data.paths.parquet_files[4].0;
                        let df = LazyFrame::scan_parquet(parquet_path.to_str().unwrap(), Default::default())
                            .expect("Failed to read matriz_riscos.parquet")
                            .collect()
                            .expect("Failed to collect LazyFrame to DataFrame");
                        data.matriz_riscos_data.0 = Some(df);
                    }
                    Box::new(MatrizRiscos::build())
                },
                "Automações" => Box::new(Automacoes::build()),
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
