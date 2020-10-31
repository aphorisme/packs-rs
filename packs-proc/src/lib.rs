use proc_macro::TokenStream;
use syn;
use crate::packable_struct_sum::impl_packable_struct_sum;
use crate::packable_struct::impl_packable_struct;
use crate::pack_unpack::{impl_pack_sum, impl_pack_struct, impl_unpack_sum, impl_unpack_struct};
use syn::DeriveInput;

mod packable_struct;
mod packable_struct_sum;
mod pack_unpack;
mod common;

#[proc_macro_derive(PackableStruct)]
pub fn packable_struct_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_packable_struct(&ast).into()
}

#[proc_macro_derive(PackableStructSum, attributes(tag))]
pub fn packable_struct_sum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_packable_struct_sum(&ast)
}

#[proc_macro_derive(Pack, attributes(tag))]
pub fn pack_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let t =
        match &ast.data {
            syn::Data::Enum(e) => impl_pack_sum(&ast.ident, &ast.generics, e),
            syn::Data::Struct(_) => impl_pack_struct(&ast.ident, &ast.generics, &ast.attrs),
            syn::Data::Union(_) => impl_pack_struct(&ast.ident, &ast.generics, &ast.attrs),
        };

    t.into()
}

#[proc_macro_derive(Unpack, attributes(tag))]
pub fn unpack_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let t =
        match &ast.data {
            syn::Data::Enum(e) => impl_unpack_sum(&ast.ident, &ast.generics, e),
            syn::Data::Struct(_) => impl_unpack_struct(&ast.ident, &ast.generics, &ast.attrs),
            syn::Data::Union(_) => impl_unpack_struct(&ast.ident, &ast.generics, &ast.attrs),
        };

    t.into()
}
