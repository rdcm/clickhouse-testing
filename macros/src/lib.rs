use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let func_vis = &func.vis;
    let func_attrs = &func.attrs;
    let func_inputs = &func.sig.inputs;
    let func_output = &func.sig.output;
    let func_block = &func.block;

    let inner_name = syn::Ident::new(&format!("__{}_inner", func_name), func_name.span());

    let expanded = quote! {
        #(#func_attrs)*
        #[tokio::test]
        #func_vis async fn #func_name() {
            use futures::FutureExt;

            let client = match clickhouse_testing::init_test(#func_name_str).await {
                Ok(c) => c,
                Err(e) => panic!("Failed to setup test {:?} client: {:?}", #func_name_str, e),
            };

            let test_result = std::panic::AssertUnwindSafe(async {
                #inner_name(client.clone()).await
            }).catch_unwind().await;

            match test_result {
                Ok(_) => {
                    if let Err(e) = clickhouse_testing::cleanup_test(&client).await {
                        panic!("Failed to cleanup test {:?} data: {:?}", #func_name_str, e);
                    }
                }
                Err(panic_info) => {
                    std::panic::resume_unwind(panic_info);
                }
            }
        }

        async fn #inner_name(#func_inputs) #func_output #func_block
    };

    expanded.into()
}
