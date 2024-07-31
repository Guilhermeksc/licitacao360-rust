use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, CellRendererText, ListStore, ScrolledWindow, TreeView, TreeViewColumn, Adjustment, WindowPosition};
use std::rc::Rc;
use std::cell::RefCell;
use polars::prelude::*;
use std::path::Path;
use calamine::{open_workbook, Reader, Xlsx};
use anyhow::{Result, anyhow};
use calamine::DataType;
use crate::utils::utf_utils::get_utf8_value;

pub struct Planejamento;

impl Planejamento {
    pub fn build(app: &gtk::Application) {
        let window = ApplicationWindow::new(app);
        window.set_title("Planejamento");
        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let dataframe = Self::create_dataframe();

        let store = ListStore::new(&[
            String::static_type(), String::static_type(), String::static_type(),
            String::static_type(), String::static_type(), String::static_type(),
            String::static_type()
        ]);
        Self::populate_store(&store, &dataframe);

        let tree_view = TreeView::with_model(&store);
        Self::setup_tree_view(&tree_view);

        let hadjustment = None::<Adjustment>;
        let vadjustment = None::<Adjustment>;
        let scrolled_window = ScrolledWindow::new(hadjustment.as_ref(), vadjustment.as_ref());

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&scrolled_window, true, true, 0);

        let add_button = Button::with_label("Adicionar");
        let delete_button = Button::with_label("Excluir");
        let save_button = Button::with_label("Salvar Tabela");
        let import_button = Button::with_label("Importar Tabela");
        let control_button = Button::with_label("Controle de Datas");

        vbox.pack_start(&add_button, false, false, 0);
        vbox.pack_start(&delete_button, false, false, 0);
        vbox.pack_start(&save_button, false, false, 0);
        vbox.pack_start(&import_button, false, false, 0);
        vbox.pack_start(&control_button, false, false, 0);

        window.add(&vbox);
        window.show_all();

        let store_clone = store.clone();
        tree_view.connect_row_activated(move |_, path, _| {
            let iter = store_clone.iter(path).unwrap();
            let selected_value: String = store_clone.value(&iter, 0).get().unwrap();
            Self::show_popup(selected_value);
        });
    }

    fn create_dataframe() -> DataFrame {
        let s1 = Series::new("id_processo", &[""; 20]);
        let s2 = Series::new("nup", &[""; 20]);
        let s3 = Series::new("objeto", &[""; 20]);
        let s4 = Series::new("valor_total", &[""; 20]);
        let s5 = Series::new("uasg", &[""; 20]);
        let s6 = Series::new("etapa", &[""; 20]);
        let s7 = Series::new("pregoeiro", &[""; 20]);
        DataFrame::new(vec![s1, s2, s3, s4, s5, s6, s7]).unwrap()
    }

    fn populate_store(store: &ListStore, dataframe: &DataFrame) {
        for i in 0..dataframe.height() {
            let id_processo = get_utf8_value(dataframe.column("id_processo").unwrap(), i);
            let nup = get_utf8_value(dataframe.column("nup").unwrap(), i);
            let objeto = get_utf8_value(dataframe.column("objeto").unwrap(), i);
            let valor_total = get_utf8_value(dataframe.column("valor_total").unwrap(), i);
            let uasg = get_utf8_value(dataframe.column("uasg").unwrap(), i);
            let etapa = get_utf8_value(dataframe.column("etapa").unwrap(), i);
            let pregoeiro = get_utf8_value(dataframe.column("pregoeiro").unwrap(), i);
            store.insert_with_values(None, &[
                (0, &id_processo),
                (1, &nup),
                (2, &objeto),
                (3, &valor_total),
                (4, &uasg),
                (5, &etapa),
                (6, &pregoeiro),
            ]);
        }
    }

    fn setup_tree_view(tree_view: &TreeView) {
        let columns = [
            ("ID Processo", 0),
            ("NUP", 1),
            ("Objeto", 2),
            ("Valor Total", 3),
            ("UASG", 4),
            ("Etapa", 5),
            ("Pregoeiro", 6),
        ];
    
        for &(title, col_id) in columns.iter() {
            let renderer = CellRendererText::new();
            let column = TreeViewColumn::new();
            column.set_title(title);
            gtk::prelude::TreeViewColumnExt::pack_start(&column, &renderer, true);
            gtk::prelude::TreeViewColumnExt::add_attribute(&column, &renderer, "text", col_id);
            tree_view.append_column(&column);
        }
    }

    fn show_popup(selected_value: String) {
        let popup = Rc::new(RefCell::new(gtk::Window::new(gtk::WindowType::Toplevel)));
        let popup_clone = Rc::clone(&popup);
        
        let label = gtk::Label::new(Some(&format!("Configurações para: {}", selected_value)));
        let button = Button::with_label("Fechar");
        button.connect_clicked(move |_| {
            popup_clone.borrow().close();
        });
        
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, false, false, 0);
        
        popup.borrow().add(&vbox);
        popup.borrow().show_all();
    }

    pub fn load_planejamento_data(path: &Path) -> Result<DataFrame, anyhow::Error> {
        let mut workbook: Xlsx<_> = open_workbook(path).map_err(|e| anyhow!("Failed to open workbook: {:?}", e))?;
        let sheet = workbook.worksheet_range("Sheet1").unwrap();

        let mut records = Vec::new();
        for row in sheet.rows().skip(1) {
            let record = PlanejamentoData {
                tipo: row.get(0).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                numero: row.get(1).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                ano: row.get(2).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                id_processo: row.get(3).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                nup: row.get(4).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                objeto: row.get(5).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                objeto_completo: row.get(6).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                valor_total: row.get(7).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                uasg: row.get(8).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                orgao_responsavel: row.get(9).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                sigla_om: row.get(10).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                setor_responsavel: row.get(11).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                coordenador_planejamento: row.get(12).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                etapa: row.get(13).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                pregoeiro: row.get(14).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                item_pca: row.get(15).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                portaria_pca: row.get(16).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                data_sessao: row.get(17).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                data_limite_entrega_tr: row.get(18).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                nup_portaria_planejamento: row.get(19).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                srp: row.get(20).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                material_servico: row.get(21).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                parecer_agu: row.get(22).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                msg_irp: row.get(23).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                data_limite_manifestacao_irp: row.get(24).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                data_limite_confirmacao_irp: row.get(25).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                num_irp: row.get(26).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                om_participantes: row.get(27).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                link_pncp: row.get(28).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                link_portal_marinha: row.get(29).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                custeio: row.get(30).and_then(|cell| cell.get_string().map(|s| s.to_string())),
                situacao: row.get(31).and_then(|cell| cell.get_string().map(|s| s.to_string())),
            };
            records.push(record);
        }

        let df = DataFrame::new(vec![
            Series::new("tipo", records.iter().map(|r| r.tipo.clone()).collect::<Vec<_>>()),
            Series::new("numero", records.iter().map(|r| r.numero.clone()).collect::<Vec<_>>()),
            Series::new("ano", records.iter().map(|r| r.ano.clone()).collect::<Vec<_>>()),
            Series::new("id_processo", records.iter().map(|r| r.id_processo.clone()).collect::<Vec<_>>()),
            Series::new("nup", records.iter().map(|r| r.nup.clone()).collect::<Vec<_>>()),
            Series::new("objeto", records.iter().map(|r| r.objeto.clone()).collect::<Vec<_>>()),
            Series::new("objeto_completo", records.iter().map(|r| r.objeto_completo.clone()).collect::<Vec<_>>()),
            Series::new("valor_total", records.iter().map(|r| r.valor_total.clone()).collect::<Vec<_>>()),
            Series::new("uasg", records.iter().map(|r| r.uasg.clone()).collect::<Vec<_>>()),
            Series::new("orgao_responsavel", records.iter().map(|r| r.orgao_responsavel.clone()).collect::<Vec<_>>()),
            Series::new("sigla_om", records.iter().map(|r| r.sigla_om.clone()).collect::<Vec<_>>()),
            Series::new("setor_responsavel", records.iter().map(|r| r.setor_responsavel.clone()).collect::<Vec<_>>()),
            Series::new("coordenador_planejamento", records.iter().map(|r| r.coordenador_planejamento.clone()).collect::<Vec<_>>()),
            Series::new("etapa", records.iter().map(|r| r.etapa.clone()).collect::<Vec<_>>()),
            Series::new("pregoeiro", records.iter().map(|r| r.pregoeiro.clone()).collect::<Vec<_>>()),
            Series::new("item_pca", records.iter().map(|r| r.item_pca.clone()).collect::<Vec<_>>()),
            Series::new("portaria_pca", records.iter().map(|r| r.portaria_pca.clone()).collect::<Vec<_>>()),
            Series::new("data_sessao", records.iter().map(|r| r.data_sessao.clone()).collect::<Vec<_>>()),
            Series::new("data_limite_entrega_tr", records.iter().map(|r| r.data_limite_entrega_tr.clone()).collect::<Vec<_>>()),
            Series::new("nup_portaria_planejamento", records.iter().map(|r| r.nup_portaria_planejamento.clone()).collect::<Vec<_>>()),
            Series::new("srp", records.iter().map(|r| r.srp.clone()).collect::<Vec<_>>()),
            Series::new("material_servico", records.iter().map(|r| r.material_servico.clone()).collect::<Vec<_>>()),
            Series::new("parecer_agu", records.iter().map(|r| r.parecer_agu.clone()).collect::<Vec<_>>()),
            Series::new("msg_irp", records.iter().map(|r| r.msg_irp.clone()).collect::<Vec<_>>()),
            Series::new("data_limite_manifestacao_irp", records.iter().map(|r| r.data_limite_manifestacao_irp.clone()).collect::<Vec<_>>()),
            Series::new("data_limite_confirmacao_irp", records.iter().map(|r| r.data_limite_confirmacao_irp.clone()).collect::<Vec<_>>()),
            Series::new("num_irp", records.iter().map(|r| r.num_irp.clone()).collect::<Vec<_>>()),
            Series::new("om_participantes", records.iter().map(|r| r.om_participantes.clone()).collect::<Vec<_>>()),
            Series::new("link_pncp", records.iter().map(|r| r.link_pncp.clone()).collect::<Vec<_>>()),
            Series::new("link_portal_marinha", records.iter().map(|r| r.link_portal_marinha.clone()).collect::<Vec<_>>()),
            Series::new("custeio", records.iter().map(|r| r.custeio.clone()).collect::<Vec<_>>()),
            Series::new("situacao", records.iter().map(|r| r.situacao.clone()).collect::<Vec<_>>()),
        ]).map_err(|e| anyhow!("Failed to create DataFrame: {:?}", e))?;

        Ok(df)
    }
}

struct PlanejamentoData {
    tipo: Option<String>,
    numero: Option<String>,
    ano: Option<String>,
    id_processo: Option<String>,
    nup: Option<String>,
    objeto: Option<String>,
    objeto_completo: Option<String>,
    valor_total: Option<String>,
    uasg: Option<String>,
    orgao_responsavel: Option<String>,
    sigla_om: Option<String>,
    setor_responsavel: Option<String>,
    coordenador_planejamento: Option<String>,
    etapa: Option<String>,
    pregoeiro: Option<String>,
    item_pca: Option<String>,
    portaria_pca: Option<String>,
    data_sessao: Option<String>,
    data_limite_entrega_tr: Option<String>,
    nup_portaria_planejamento: Option<String>,
    srp: Option<String>,
    material_servico: Option<String>,
    parecer_agu: Option<String>,
    msg_irp: Option<String>,
    data_limite_manifestacao_irp: Option<String>,
    data_limite_confirmacao_irp: Option<String>,
    num_irp: Option<String>,
    om_participantes: Option<String>,
    link_pncp: Option<String>,
    link_portal_marinha: Option<String>,
    custeio: Option<String>,
    situacao: Option<String>,
}
