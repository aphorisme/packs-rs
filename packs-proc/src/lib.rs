use proc_macro::TokenStream;
use syn;
use crate::pack_unpack::{impl_pack_sum, impl_pack_struct, impl_unpack_sum, impl_unpack_struct};
use syn::DeriveInput;

mod pack_unpack;
mod common;

#[proc_macro_derive(Pack, attributes(tag))]
pub fn pack_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let t =
        match &ast.data {
            syn::Data::Enum(e) => impl_pack_sum(&ast.ident, &ast.generics, e),
            syn::Data::Struct(s) => impl_pack_struct(&ast.ident, &ast.generics, &ast.attrs, s),
            _ => panic!("Only enums and structs are supported for deriving Pack."),
        };

    t.into()
}

#[proc_macro_derive(Unpack, attributes(tag))]
pub fn unpack_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let t =
        match &ast.data {
            syn::Data::Enum(e) => impl_unpack_sum(&ast.ident, &ast.generics, e),
            syn::Data::Struct(s) => impl_unpack_struct(&ast.ident, &ast.generics, &ast.attrs, s),
            _ => panic!("Only enums and structs are supported for deriving Unpack."),
        };

    t.into()
}
