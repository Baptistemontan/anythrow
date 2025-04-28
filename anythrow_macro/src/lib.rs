mod try_catch;

#[proc_macro_attribute]
pub fn try_catch(
    _attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::ItemFn);
    try_catch::try_catch(input, false).into()
}
