use polars::prelude::*;
use druid::Data;

#[derive(Clone)]
pub struct DataFrameWrapper(pub Option<DataFrame>);

impl Data for DataFrameWrapper {
    fn same(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(df1), Some(df2)) => {
                // Aqui você pode comparar os DataFrames como desejar
                df1.shape() == df2.shape() // ou qualquer outra comparação
            }
            (None, None) => true,
            _ => false,
        }
    }
}
