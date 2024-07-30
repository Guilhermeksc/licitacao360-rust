use std::fs::{self, File};
use std::path::{Path, PathBuf};
use druid::Data;
use polars::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DataPath(pub PathBuf);

impl Data for DataPath {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Paths {
    pub base_path: DataPath,
    pub database_dir: DataPath,
    pub icons_path: DataPath,
    pub images_path: DataPath,
    pub contratos_path: DataPath,
    pub planejamento_path: DataPath,
    pub atas_path: DataPath,
    pub dispensa_eletronica_path: DataPath,
    pub matriz_riscos_path: DataPath,
    pub automacoes_path: DataPath,
    pub inicio_path: DataPath,
    pub parquet_files: Vec<DataPath>,
}

impl Data for Paths {
    fn same(&self, other: &Self) -> bool {
        self.base_path.same(&other.base_path) &&
        self.database_dir.same(&other.database_dir) &&
        self.icons_path.same(&other.icons_path) &&
        self.images_path.same(&other.images_path) &&
        self.contratos_path.same(&other.contratos_path) &&
        self.planejamento_path.same(&other.planejamento_path) &&
        self.atas_path.same(&other.atas_path) &&
        self.dispensa_eletronica_path.same(&other.dispensa_eletronica_path) &&
        self.matriz_riscos_path.same(&other.matriz_riscos_path) &&
        self.automacoes_path.same(&other.automacoes_path) &&
        self.inicio_path.same(&other.inicio_path) &&
        self.parquet_files.len() == other.parquet_files.len() &&
        self.parquet_files.iter().zip(&other.parquet_files).all(|(a, b)| a.same(b))
    }
}

impl Paths {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let base_path = DataPath(base_path.as_ref().to_path_buf());
        let database_dir = DataPath(base_path.0.join("src/database"));

        let parquet_files = vec![
            DataPath(database_dir.0.join("contratos.parquet")),
            DataPath(database_dir.0.join("planejamento.parquet")),
            DataPath(database_dir.0.join("atas.parquet")),
            DataPath(database_dir.0.join("dispensa_eletronica.parquet")),
            DataPath(database_dir.0.join("matriz_riscos.parquet")),
        ];

        Self {
            base_path: base_path.clone(),
            database_dir,
            icons_path: DataPath(base_path.0.join("src/resources/icons")),
            images_path: DataPath(base_path.0.join("src/resources/images")),
            contratos_path: DataPath(base_path.0.join("src/modules/contratos/templates")),
            planejamento_path: DataPath(base_path.0.join("src/modules/planejamento")),
            atas_path: DataPath(base_path.0.join("src/modules/atas")),
            dispensa_eletronica_path: DataPath(base_path.0.join("src/modules/dispensa_eletronica")),
            matriz_riscos_path: DataPath(base_path.0.join("src/modules/matriz_riscos")),
            automacoes_path: DataPath(base_path.0.join("src/modules/automacoes")),
            inicio_path: DataPath(base_path.0.join("src/modules/inicio")),
            parquet_files,
        }
    }

    pub fn icon_path(&self, icon_name: &str) -> PathBuf {
        self.icons_path.0.join(icon_name)
    }

    pub fn image_path(&self, image_name: &str) -> PathBuf {
        self.images_path.0.join(image_name)
    }

    pub fn menu_image_path(&self) -> PathBuf {
        self.images_path.0.join("licitacao360.png")
    }

    pub fn database_init(&self) -> PathBuf {
        self.database_dir.0.clone()
    }

    pub fn create_database_dirs(&self) {
        let db_dir = self.database_init();
        if !db_dir.exists() {
            fs::create_dir_all(&db_dir).expect("Failed to create database directory");
        }
    }

    pub fn create_parquet_files(&self) {
        for parquet_file in &self.parquet_files {
            if !parquet_file.0.exists() {
                let mut df = DataFrame::new(vec![] as Vec<Series>).expect("Failed to create DataFrame");
                let file = File::create(&parquet_file.0).expect("Failed to create Parquet file");
                ParquetWriter::new(file).finish(&mut df).expect("Failed to write Parquet file");
            }
        }
    }
}
