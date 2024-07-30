// paths.rs

use std::path::{Path, PathBuf};
use druid::Data;

#[derive(Clone, PartialEq)]
struct DataPath(PathBuf);

impl Data for DataPath {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Data, PartialEq)]
pub struct Paths {
    base_path: DataPath,
    icons_path: DataPath,
    images_path: DataPath,
    templates_contratos_path: DataPath,
    planejamento_path: DataPath,
    atas_path: DataPath,
    dispensa_eletronica_path: DataPath,
    matriz_riscos_path: DataPath,
    automacoes_path: DataPath,
    inicio_path: DataPath,
}

impl Paths {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let base_path = DataPath(base_path.as_ref().to_path_buf());
        let icons_path = DataPath(base_path.0.join("src\\resources\\icons"));
        let images_path = DataPath(base_path.0.join("src\\resources\\images"));
        let templates_contratos_path = DataPath(base_path.0.join("src\\modules\\contratos\\templates"));
        let planejamento_path = DataPath(base_path.0.join("src\\modules\\planejamento"));
        let atas_path = DataPath(base_path.0.join("src\\modules\\atas"));
        let dispensa_eletronica_path = DataPath(base_path.0.join("src\\modules\\dispensa_eletronica"));
        let matriz_riscos_path = DataPath(base_path.0.join("src\\modules\\matriz_riscos"));
        let automacoes_path = DataPath(base_path.0.join("src\\modules\\automacoes"));
        let inicio_path = DataPath(base_path.0.join("src\\modules\\inicio"));       

        Self {
            base_path,
            icons_path,
            images_path,
            templates_contratos_path,
            dispensa_eletronica_path,
            planejamento_path,
            atas_path,
            matriz_riscos_path,
            automacoes_path,
            inicio_path,            
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
}
