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
    pub templates_paths: Vec<DataPath>,
    pub inicio_path: DataPath,
    pub parquet_files: Vec<DataPath>,
}

impl Data for Paths {
    fn same(&self, other: &Self) -> bool {
        self.base_path.same(&other.base_path) &&
        self.database_dir.same(&other.database_dir) &&
        self.icons_path.same(&other.icons_path) &&
        self.images_path.same(&other.images_path) &&
        self.templates_paths == other.templates_paths &&
        self.inicio_path.same(&other.inicio_path) &&
        self.parquet_files == other.parquet_files
    }
}

impl Paths {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let base_path = DataPath(base_path.as_ref().to_path_buf());
        let database_dir = DataPath(base_path.0.join("src/database"));

        let parquet_files = vec![
            "contratos.parquet",
            "planejamento.parquet",
            "atas.parquet",
            "dispensa_eletronica.parquet",
            "matriz_riscos.parquet",
            "automacoes.parquet"
        ].into_iter()
            .map(|file| DataPath(database_dir.0.join(file)))
            .collect();

        let templates_paths = vec![
            "src/modules/contratos/templates",
            "src/modules/planejamento/templates",
            "src/modules/atas/templates",
            "src/modules/dispensa_eletronica/templates",
            "src/modules/matriz_riscos/templates",
            "src/modules/automacoes/templates"
        ].into_iter()
            .map(|dir| DataPath(base_path.0.join(dir)))
            .collect();

        Self {
            base_path: base_path.clone(),
            database_dir,
            icons_path: DataPath(base_path.0.join("src/resources/icons")),
            images_path: DataPath(base_path.0.join("src/resources/images")),
            templates_paths,
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

    pub fn atas_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("atas.parquet")
    }

    pub fn matriz_riscos_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("matriz_riscos.parquet")
    }

    pub fn planejamento_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("planejamento.parquet")
    }

    pub fn contratos_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("contratos.parquet")
    }

    pub fn dispensa_eletronica_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("dispensa_eletronica.parquet")
    }

    pub fn automacoes_parquet_path(&self) -> PathBuf {
        self.database_dir.0.join("automacoes.parquet")
    }
}
    