use syn::Attribute;
use proc_macro2::{Span, TokenStream};
use quote::quote;


pub fn gen_type_param() -> syn::Ident {
    syn::Ident::new("T__PACKS_PROC_IMPL_PACKUNPACK", Span::call_site())
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