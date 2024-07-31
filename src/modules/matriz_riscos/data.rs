use polars::prelude::*;
use std::fs::File;
use crate::paths::Paths;
use crate::utils::utils_load::load_or_create;

#[derive(Clone, Debug)]
pub struct MatrizRiscosData {
    pub risco: Option<String>,
    pub causa: Option<String>,
    pub consequencia: Option<String>,
    pub acao_corretiva: Option<String>,
    pub acao_preventiva: Option<String>,
    pub impacto: Option<String>,
    pub probabilidade: Option<String>,
}

impl MatrizRiscosData {
    pub fn new() -> Self {
        MatrizRiscosData {
            risco: None,
            causa: None,
            consequencia: None,
            acao_corretiva: None,
            acao_preventiva: None,
            impacto: None,
            probabilidade: None,
        }
    }

    pub fn load_or_create(paths: &Paths) -> DataFrame {
        load_or_create(paths, "matriz_riscos.parquet", vec![
            Series::new("risco", Vec::<&str>::new()),
            Series::new("causa", Vec::<&str>::new()),
            Series::new("consequencia", Vec::<&str>::new()),
            Series::new("acao_corretiva", Vec::<&str>::new()),
            Series::new("acao_preventiva", Vec::<&str>::new()),
            Series::new("impacto", Vec::<&str>::new()),
            Series::new("probabilidade", Vec::<&str>::new()),
        ])
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let risco_series = Series::new("risco", vec![self.risco.clone()]);
        let causa_series = Series::new("causa", vec![self.causa.clone()]);
        let consequencia_series = Series::new("consequencia", vec![self.consequencia.clone()]);
        let acao_corretiva_series = Series::new("acao_corretiva", vec![self.acao_corretiva.clone()]);
        let acao_preventiva_series = Series::new("acao_preventiva", vec![self.acao_preventiva.clone()]);
        let impacto_series = Series::new("impacto", vec![self.impacto.clone()]);
        let probabilidade_series = Series::new("probabilidade", vec![self.probabilidade.clone()]);

        DataFrame::new(vec![
            risco_series,
            causa_series,
            consequencia_series,
            acao_corretiva_series,
            acao_preventiva_series,
            impacto_series,
            probabilidade_series,
        ]).expect("Failed to create DataFrame")
    }

    pub fn save_to_parquet(&self, paths: &Paths) -> Result<(), PolarsError> {
        let mut df = self.to_dataframe();
        let file = File::create(paths.matriz_riscos_parquet_path())?;
        ParquetWriter::new(file).finish(&mut df).map(|_| ())
    }
}