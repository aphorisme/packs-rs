use proc_macro2::{TokenStream};
use crate::common::{get_tag_attr, gen_type_param, gen_packable_struct_sum_constraint};
use quote::quote;

pub fn impl_pack(ast: &syn::DeriveInput) -> TokenStream {
    let tag = get_tag_attr(&ast.attrs).expect("Need #[tag = u8] attribute on struct.");
    let ident = &ast.ident;
    let ty_write = gen_type_param();
    let generics = &ast.generics;
    let constraint_sum = gen_packable_struct_sum_constraint(generics);

    quote! {
        impl<#ty_write: Write, #constraint_sum> Pack<#ty_write> for #ident #generics {
            fn encode(&self, writer: &mut #ty_write) -> Result<usize, EncodeError> {
                encode_struct(self, #tag, writer)
            }
        }
    }
}

pub fn impl_unpack(ast: &syn::DeriveInput) -> TokenStream {
    let tag = get_tag_attr(&ast.attrs).expect("Need #[tag = u8] attribute on struct.");
    let ident = &ast.ident;
    let ty_read = gen_type_param();
    let generics = &ast.generics;
    let constraint_sum = gen_packable_struct_sum_constraint(generics);
    quote! {
        impl<#ty_read: Read, #constraint_sum> Unpack<#ty_read> for #ident #generics {
            fn decode(reader: &mut #ty_read) -> Result<Self, DecodeError> {
                decode_struct(#tag, reader)
            }
        }
    }
}
