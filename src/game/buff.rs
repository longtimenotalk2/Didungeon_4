
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum Buff {
  Surrender
}

impl Buff {
  pub fn to_string(&self) -> String {
    match self {
      Buff::Surrender => "投降".to_string(),
    }
  }

  pub fn all() -> Vec<Self> {
    vec![Self::Surrender]
  }
}