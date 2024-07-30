// src/modules/matriz_riscos/model.rs

// Aqui você pode adicionar estruturas de dados, enumerações e funções que lidam com a lógica do módulo.
pub struct Risk {
    pub name: String,
    pub impact: f64,
    pub probability: f64,
}

impl Risk {
    pub fn new(name: &str, impact: f64, probability: f64) -> Self {
        Self {
            name: name.to_string(),
            impact,
            probability,
        }
    }

    pub fn calculate_risk(&self) -> f64 {
        self.impact * self.probability
    }
}
