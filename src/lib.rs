#[macro_use]
pub mod macros;

pub mod core;
pub mod x;

decl_data! {
  pub struct Data {
    pub coin_gecko: f32 = x::coingecko::Price::new("bitcoin"),
    pub crypto_compare: f32 = x::cryptocompare::Price::new(),
  }
}
