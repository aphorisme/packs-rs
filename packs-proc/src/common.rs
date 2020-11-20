use proc_macro2::Span;
use syn::{Attribute, Type, Variant};

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

pub fn get_tag_attr(attributes: &[Attribute]) -> Option<u8> {
    get_name_value_attr("tag", attributes).map(lit_to_u8)
}

pub fn get_fields_attr(attributes: &[Attribute]) -> Option<usize> {
    get_name_value_attr("fields", attributes).map(lit_to_usize)
}

pub fn get_pack_attr_param(attributes: &[Attribute]) -> Option<syn::Ident> {
    let attr = get_attr("pack", attributes)?;
    attr.parse_args().ok()
}

pub fn get_unpack_attr_param(attributes: &[Attribute]) -> Option<syn::Ident> {
    let attr = get_attr("unpack", attributes)?;
    attr.parse_args().ok()
}

pub fn get_attr<'a>(attr_name: &str, attributes: &'a [Attribute]) -> Option<&'a Attribute> {
    for attr in attributes {
        if let Some(ident) = attr.path.get_ident() {
            if ident == attr_name {
                return Some(attr)
            }
        }
    }

    None
}

pub fn get_name_value_attr(attr_name: &str, attributes: &[Attribute]) -> Option<syn::Lit> {
    for attr in attributes {
        if let syn::Meta::NameValue(nv) = attr.parse_meta().unwrap() {
            if let Some(ident) = nv.path.get_ident() {
                if ident == attr_name {
                    return Some(nv.lit)
                }
            }
        }
    }

    None
}

fn lit_to_u8(lit: syn::Lit) -> u8 {
    match lit {
        syn::Lit::Int(li) => {
            li.base10_parse().unwrap()
        }
        syn::Lit::Byte(lb) => {
            lb.value()
        }
        _ => panic!("Cannot parse into u8")
    }
}

fn lit_to_usize(lit: syn::Lit) -> usize {
    match lit {
        syn::Lit::Int(li) => {
            li.base10_parse().unwrap()
        },
        _ => panic!("Cannot parse into usize")
    }
}