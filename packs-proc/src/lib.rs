use proc_macro::TokenStream;
use syn::DeriveInput;
use pack::{impl_pack_sum, impl_pack_struct};
use unpack::{impl_unpack_sum, impl_unpack_struct};

mod pack;
mod unpack;
mod common;

#[proc_macro_derive(Pack, attributes(tag, pack, fields))]
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

#[proc_macro_derive(Unpack, attributes(tag, unpack, fields))]
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
