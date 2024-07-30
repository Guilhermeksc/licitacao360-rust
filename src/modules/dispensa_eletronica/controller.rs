
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Serialize, Deserialize)]
pub struct DispensaEletronicaController {
    // Adicione os campos específicos do controller aqui
    pub another_field: String,
}

impl Default for DispensaEletronicaController {
    fn default() -> Self {
        Self {
            another_field: "default value".into(),
        }
    }
}
