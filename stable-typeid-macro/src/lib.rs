use quote::ToTokens as _;

mod util;

fn expand_derive_stable_id(input: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let type_string = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                let type_str_list: Vec<String> = fields
                    .named
                    .iter()
                    .map(|f| match &f.ident {
                        Some(ident) => format!("{ident}: {}", f.ty.to_token_stream()),
                        None => {
                            format!("{}", f.ty.to_token_stream())
                        }
                    })
                    .collect();
                format!("struct~{}{{{}}}", name, type_str_list.join(";"))
            }
            syn::Fields::Unit => {
                format!("struct~{}", name)
            }
            syn::Fields::Unnamed(fields) => {
                let type_str_list: Vec<String> = fields
                    .unnamed
                    .iter()
                    .map(|f| match &f.ident {
                        Some(ident) => format!("{ident}: {}", f.ty.to_token_stream()),
                        None => {
                            format!("{}", f.ty.to_token_stream())
                        }
                    })
                    .collect();
                format!("struct~{}({})", name, type_str_list.join(","))
            }
        },
        syn::Data::Enum(data) => {
            let type_str_list: Vec<String> = data
                .variants
                .iter()
                .map(|v| format!("{}{}", v.ident, v.fields.to_token_stream(),))
                .collect();
            format!("enum~{}{{{}}}", name, type_str_list.join(","))
        }
        syn::Data::Union(_) => {
            panic!("#[derive(stable_typeid::StableID)] can only be implemented on struct or enum")
        }
    };
    let type_string = format!("{}%{}", util::get_pkg_name(), type_string);
    let hash = util::hash(&type_string);
    let doc = format!("type_name = {} \ntype_id = {}", type_string, hash);

    let impl_block = quote::quote! {
        #[doc = #doc]
        impl stable_typeid::StableAny for #name {
            fn stable_id(&self) -> &'static stable_typeid::StableId where Self: Sized {
                &stable_typeid::StableId(#hash)
            }
        }

        impl stable_typeid::StableID for #name {
            const _STABLE_ID: &'static stable_typeid::StableId = &stable_typeid::StableId(#hash);
        }
    };
    Ok(impl_block)
}

#[proc_macro_derive(StableID)]
pub fn stable_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    expand_derive_stable_id(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
