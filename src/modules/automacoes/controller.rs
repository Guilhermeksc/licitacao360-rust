

use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Serialize, Deserialize)]
pub struct AutomacoesController {
    // Adicione os campos específicos do controller aqui
    pub another_field: String,
}

impl Default for AutomacoesController {
    fn default() -> Self {
        Self {
            another_field: "default value".into(),
        }
    }
}
