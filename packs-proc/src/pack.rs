use proc_macro2::{Ident, TokenStream};
use syn::{Generics, Attribute, DataStruct};
use crate::common::{get_fields_attr, get_tag_attr, get_pack_attr_param, gen_type_param, get_singleton_field_type};
use quote::quote;

pub fn impl_pack_struct(ident: &Ident, generics: &Generics, attrs: &[Attribute], s: &DataStruct) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_write = gen_type_param();

    let mut pack_cases = proc_macro2::TokenStream::new();
    let mut fields = 0usize;

    for f in &s.fields {
        let field_ident =
            f.ident.as_ref().expect("Expected identifier at field.");
        let field_type = &f.ty;
        // use the #[fields = usize] attribute if given:
        fields += get_fields_attr(&f.attrs).unwrap_or(1);

        let pack =
            if let Some(ident) = get_pack_attr_param(&f.attrs) {
                // with #[pack(func)] attribute:
                quote! {
                    written += #ident(&self.#field_ident, writer)?;
                 }
            } else {
                // without:
                quote! {
                    written += <#field_type as Pack>::encode(&self.#field_ident, writer)?;
                }
            };

        pack_cases.extend(pack);
    }

    if fields > 15 {
        panic!("More then 15 fields are not allowed for a struct.");
    }

    quote! {
        impl #generics Pack for #ident #generics {
            fn encode<#ty_write: std::io::Write>(&self, writer: &mut #ty_write) -> Result<usize, EncodeError> {
                let mut written =
                    Marker::Structure(#fields, #tag).encode(writer)?;
                #pack_cases

                Ok(written)
            }
        }
    }
}

pub fn impl_pack_sum(ident: &Ident, generics: &Generics, ast: &syn::DataEnum) -> TokenStream {
    let mut pack_cases = proc_macro2::TokenStream::new();

    let ty_param = gen_type_param();

    for v in ast.variants.iter() {
        let var_name = &v.ident;
        let var_type = get_singleton_field_type(v);

        pack_cases.extend(quote! {
            #ident::#var_name(v) => {
                <#var_type as Pack>::encode(v, writer)
            },
        });
    }


    quote! {
        impl #generics Pack for #ident #generics  {
            fn encode<#ty_param: std::io::Write>(&self, writer: &mut #ty_param) -> Result<usize, EncodeError> {
                match self {
                    #pack_cases
                }
            }
        }
    }
}
