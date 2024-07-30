use serde::{Deserialize, Serialize};
use polars::prelude::*;
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;
use anyhow::{Result, anyhow};
use druid::Data;
use std::fs::File;

#[derive(Clone, Data, Serialize, Deserialize, Debug)]
pub struct TableData {
    pub rows: Vector<TableRow>, // Usando Vector em vez de Vec para compatibilidade com druid::Data
}

#[derive(Clone, Data, Serialize, Deserialize, Debug)]
pub struct TableRow {
    pub values: Vector<String>, // Usando Vector em vez de Vec para compatibilidade com druid::Data
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanejamentoData {
    pub tipo: Option<String>,
    pub numero: Option<String>,
    pub ano: Option<String>,
    pub id_processo: Option<String>,
    pub nup: Option<String>,
    pub objeto: Option<String>,
    pub objeto_completo: Option<String>,
    pub valor_total: Option<String>,
    pub uasg: Option<String>,
    pub orgao_responsavel: Option<String>,
    pub sigla_om: Option<String>,
    pub setor_responsavel: Option<String>,
    pub coordenador_planejamento: Option<String>,
    pub etapa: Option<String>,
    pub pregoeiro: Option<String>,
    pub item_pca: Option<String>,
    pub portaria_pca: Option<String>,
    pub data_sessao: Option<String>,
    pub data_limite_entrega_tr: Option<String>,
    pub nup_portaria_planejamento: Option<String>,
    pub srp: Option<String>,
    pub material_servico: Option<String>,
    pub parecer_agu: Option<String>,
    pub msg_irp: Option<String>,
    pub data_limite_manifestacao_irp: Option<String>,
    pub data_limite_confirmacao_irp: Option<String>,
    pub num_irp: Option<String>,
    pub om_participantes: Option<String>,
    pub link_pncp: Option<String>,
    pub link_portal_marinha: Option<String>,
    pub custeio: Option<String>,
    pub situacao: Option<String>,
}

pub fn load_planejamento_data(path: &Path) -> Result<DataFrame> {
    let mut workbook: Xlsx<_> = open_workbook(path).map_err(|e| anyhow!("Failed to open workbook: {:?}", e))?;
    let sheet = workbook.worksheet_range("Sheet1").unwrap();

    let mut records = Vec::new();
    for row in range.rows().skip(1) {
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