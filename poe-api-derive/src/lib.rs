use poe_api_core::gqlmodel_core;
use proc_macro::TokenStream;

#[proc_macro_derive(GQLModel, attributes(gql))]
// note it's proc_macro1 token stream
pub fn gqlmodel_derive_macro2(item: TokenStream) -> TokenStream {
    gqlmodel_core(item.into()).unwrap().into()
}
