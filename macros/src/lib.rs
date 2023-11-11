use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ScrollerGenerator)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
  let DeriveInput { ident, .. } = parse_macro_input!(item);

  format!("impl ScrollerGenerator for {ident} {{}}")
    .parse()
    .unwrap()
}
