use polars::prelude::*;
use std::fs::File;
use crate::paths::Paths;
use crate::utils::utils_load::load_or_create;

#[derive(Clone, Debug)]
pub struct ContratosData {
    pub numero: Option<String>,
    pub ano: Option<String>,
    pub id_processo: Option<String>,
    pub nup: Option<String>,
    pub objeto: Option<String>,
}

impl ContratosData {
    pub fn new() -> Self {
        ContratosData {
            numero: None,
            ano: None,
            id_processo: None,
            nup: None,
            objeto: None,
        }
    }

    pub fn load_or_create(paths: &Paths) -> DataFrame {
        load_or_create(paths, "contratos.parquet", vec![
            Series::new("numero", Vec::<&str>::new()),
            Series::new("ano", Vec::<&str>::new()),
            Series::new("id_processo", Vec::<&str>::new()),
            Series::new("nup", Vec::<&str>::new()),
            Series::new("objeto", Vec::<&str>::new()),
        ])
    }

    pub fn to_dataframe(&self) -> DataFrame {
        let numero_series = Series::new("numero", vec![self.numero.clone()]);
        let ano_series = Series::new("ano", vec![self.ano.clone()]);
        let id_processo_series = Series::new("id_processo", vec![self.id_processo.clone()]);
        let nup_series = Series::new("nup", vec![self.nup.clone()]);
        let objeto_series = Series::new("objeto", vec![self.objeto.clone()]);

        DataFrame::new(vec![
            numero_series,
            ano_series,
            id_processo_series,
            nup_series,
            objeto_series,
        ]).expect("Failed to create DataFrame")
    }

    pub fn save_to_parquet(&self, paths: &Paths) -> Result<(), PolarsError> {
        let mut df = self.to_dataframe();
        let file = File::create(paths.contratos_parquet_path())?;
        ParquetWriter::new(file).finish(&mut df).map(|_| ())
    }
}
