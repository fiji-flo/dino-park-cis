use serde::Deserialize;

#[derive(Deserialize)]
pub enum DisplayFilter {
    True,
    False,
    Any
}

impl DisplayFilter {
    pub fn filter(&self) -> &[bool] {
        match self {
            Self::True => &[true],
            Self::False => &[false],
            Self::Any => &[true, false],
        }
    }
}