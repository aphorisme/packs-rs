use syn::{Attribute, Variant, Type};
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub mod enums;

pub fn gen_type_param() -> syn::Ident {
    syn::Ident::new("T__PACKS_PROC_IMPL_PACKUNPACK", Span::call_site())
}

pub fn get_singleton_field_type(v: &Variant) -> &Type {
    if v.fields.len() != 1 {
        panic!("Variant '{}' has != 1 fields", v.ident)
    } else {
        &v.fields.iter().next().unwrap().ty
    }
}

pub fn gen_packable_struct_sum_constraint(generics: &syn::Generics) -> TokenStream {
    if let Some(ty_param) = generics.type_params().next() {
        let ident = &ty_param.ident;
        quote! { #ident: PackableStructSum }
    } else {
        TokenStream::new()
    }
}

pub fn get_tag_attr(attributes: &Vec<Attribute>) -> Option<u8> {
    for attr in attributes {
        let meta = attr.parse_meta().unwrap();
        match meta {
            syn::Meta::NameValue(nv) => {
                if let Some(ident) = nv.path.get_ident() {
                    if ident == "tag" {
                        return Some(lit_to_u8(nv.lit));
                    }
                }
            },

            _ => ()
        }
    }

    None
}

fn lit_to_u8(lit: syn::Lit) -> u8 {
    match lit {
        syn::Lit::Int(li) => {
            li.base10_parse().unwrap()
        },
        syn::Lit::Byte(lb) => {
            lb.value()
        },
        _ => panic!("Cannot parse into u8")
    }
}