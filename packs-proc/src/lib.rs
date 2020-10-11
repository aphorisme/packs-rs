use proc_macro::TokenStream;
use syn;
use crate::packable_struct_sum::impl_packable_struct_sum;
use crate::packable_struct::impl_packable_struct;
use crate::pack_unpack::{impl_pack, impl_unpack};

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
    let ast = syn::parse(input).unwrap();

    impl_pack(&ast).into()
}

#[proc_macro_derive(Unpack, attributes(tag))]
pub fn unpack_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_unpack(&ast).into()
}
