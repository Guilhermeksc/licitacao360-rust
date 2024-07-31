// src/modules/planejamento/data.rs

use polars::prelude::*;
use std::fs::File;
use crate::paths::Paths;
use crate::utils::utils_load::load_or_create;

#[derive(Clone, Debug)]
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

impl PlanejamentoData {
    pub fn new() -> Self {
        PlanejamentoData {
            tipo: None,
            numero: None,
            ano: None,
            id_processo: None,
            nup: None,
            objeto: None,
            objeto_completo: None,
            valor_total: None,
            uasg: None,
            orgao_responsavel: None,
            sigla_om: None,
            setor_responsavel: None,
            coordenador_planejamento: None,
            etapa: None,
            pregoeiro: None,
            item_pca: None,
            portaria_pca: None,
            data_sessao: None,
            data_limite_entrega_tr: None,
            nup_portaria_planejamento: None,
            srp: None,
            material_servico: None,
            parecer_agu: None,
            msg_irp: None,
            data_limite_manifestacao_irp: None,
            data_limite_confirmacao_irp: None,
            num_irp: None,
            om_participantes: None,
            link_pncp: None,
            link_portal_marinha: None,
            custeio: None,
            situacao: None,
        }
    }

    pub fn load_or_create(paths: &Paths) -> DataFrame {
        load_or_create(paths, "planejamento.parquet", vec![
            Series::new("tipo", Vec::<&str>::new()),
            Series::new("numero", Vec::<&str>::new()),
            Series::new("ano", Vec::<&str>::new()),
            Series::new("id_processo", Vec::<&str>::new()),
            Series::new("nup", Vec::<&str>::new()),
            Series::new("objeto", Vec::<&str>::new()),
            Series::new("objeto_completo", Vec::<&str>::new()),
            Series::new("valor_total", Vec::<&str>::new()),
            Series::new("uasg", Vec::<&str>::new()),
            Series::new("orgao_responsavel", Vec::<&str>::new()),
            Series::new("sigla_om", Vec::<&str>::new()),
            Series::new("setor_responsavel", Vec::<&str>::new()),
            Series::new("coordenador_planejamento", Vec::<&str>::new()),
            Series::new("etapa", Vec::<&str>::new()),
            Series::new("pregoeiro", Vec::<&str>::new()),
            Series::new("item_pca", Vec::<&str>::new()),
            Series::new("portaria_pca", Vec::<&str>::new()),
            Series::new("data_sessao", Vec::<&str>::new()),
            Series::new("data_limite_entrega_tr", Vec::<&str>::new()),
            Series::new("nup_portaria_planejamento", Vec::<&str>::new()),
            Series::new("srp", Vec::<&str>::new()),
            Series::new("material_servico", Vec::<&str>::new()),
            Series::new("parecer_agu", Vec::<&str>::new()),
            Series::new("msg_irp", Vec::<&str>::new()),
            Series::new("data_limite_manifestacao_irp", Vec::<&str>::new()),
            Series::new("data_limite_confirmacao_irp", Vec::<&str>::new()),
            Series::new("num_irp", Vec::<&str>::new()),
            Series::new("om_participantes", Vec::<&str>::new()),
            Series::new("link_pncp", Vec::<&str>::new()),
            Series::new("link_portal_marinha", Vec::<&str>::new()),
            Series::new("custeio", Vec::<&str>::new()),
            Series::new("situacao", Vec::<&str>::new()),
        ])
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let tipo_series = Series::new("tipo", vec![self.tipo.clone()]);
        let numero_series = Series::new("numero", vec![self.numero.clone()]);
        let ano_series = Series::new("ano", vec![self.ano.clone()]);
        let id_processo_series = Series::new("id_processo", vec![self.id_processo.clone()]);
        let nup_series = Series::new("nup", vec![self.nup.clone()]);
        let objeto_series = Series::new("objeto", vec![self.objeto.clone()]);
        let objeto_completo_series = Series::new("objeto_completo", vec![self.objeto_completo.clone()]);
        let valor_total_series = Series::new("valor_total", vec![self.valor_total.clone()]);
        let uasg_series = Series::new("uasg", vec![self.uasg.clone()]);
        let orgao_responsavel_series = Series::new("orgao_responsavel", vec![self.orgao_responsavel.clone()]);
        let sigla_om_series = Series::new("sigla_om", vec![self.sigla_om.clone()]);
        let setor_responsavel_series = Series::new("setor_responsavel", vec![self.setor_responsavel.clone()]);
        let coordenador_planejamento_series = Series::new("coordenador_planejamento", vec![self.coordenador_planejamento.clone()]);
        let etapa_series = Series::new("etapa", vec![self.etapa.clone()]);
        let pregoeiro_series = Series::new("pregoeiro", vec![self.pregoeiro.clone()]);
        let item_pca_series = Series::new("item_pca", vec![self.item_pca.clone()]);
        let portaria_pca_series = Series::new("portaria_pca", vec![self.portaria_pca.clone()]);
        let data_sessao_series = Series::new("data_sessao", vec![self.data_sessao.clone()]);
        let data_limite_entrega_tr_series = Series::new("data_limite_entrega_tr", vec![self.data_limite_entrega_tr.clone()]);
        let nup_portaria_planejamento_series = Series::new("nup_portaria_planejamento", vec![self.nup_portaria_planejamento.clone()]);
        let srp_series = Series::new("srp", vec![self.srp.clone()]);    
        let material_servico_series = Series::new("material_servico", vec![self.material_servico.clone()]);
        let parecer_agu_series = Series::new("parecer_agu", vec![self.parecer_agu.clone()]);
        let msg_irp_series = Series::new("msg_irp", vec![self.msg_irp.clone()]);
        let data_limite_manifestacao_irp_series = Series::new("data_limite_manifestacao_irp", vec![self.data_limite_manifestacao_irp.clone()]);
        let data_limite_confirmacao_irp_series = Series::new("data_limite_confirmacao_irp", vec![self.data_limite_confirmacao_irp.clone()]);
        let num_irp_series = Series::new("num_irp", vec![self.num_irp.clone()]);
        let om_participantes_series = Series::new("om_participantes", vec![self.om_participantes.clone()]);
        let link_pncp_series = Series::new("link_pncp", vec![self.link_pncp.clone()]);
        let link_portal_marinha_series = Series::new("link_portal_marinha", vec![self.link_portal_marinha.clone()]);
        let custeio_series = Series::new("custeio", vec![self.custeio.clone()]);
        let situacao_series = Series::new("situacao", vec![self.situacao.clone()]);

        DataFrame::new(vec![
            tipo_series,
            numero_series,
            ano_series,
            id_processo_series,
            nup_series,
            objeto_series,
            objeto_completo_series,
            valor_total_series,
            uasg_series,
            orgao_responsavel_series,
            sigla_om_series,
            setor_responsavel_series,
            coordenador_planejamento_series,
            etapa_series,
            pregoeiro_series,
            item_pca_series,
            portaria_pca_series,
            data_sessao_series,
            data_limite_entrega_tr_series,
            nup_portaria_planejamento_series,
            srp_series,
            material_servico_series,
            parecer_agu_series,
            msg_irp_series,
            data_limite_manifestacao_irp_series,
            data_limite_confirmacao_irp_series,
            num_irp_series,
            om_participantes_series,
            link_pncp_series,
            link_portal_marinha_series,
            custeio_series,
            situacao_series,
        ]).expect("Failed to create DataFrame")
    }

    pub fn save_to_parquet(&self, paths: &Paths) -> Result<(), PolarsError> {
        let mut df = self.to_dataframe();
        let file = File::create(paths.planejamento_parquet_path())?;
        ParquetWriter::new(file).finish(&mut df).map(|_| ())
    }
}