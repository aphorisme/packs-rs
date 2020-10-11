use proc_macro2::TokenStream;
use quote::quote;
use crate::common::{gen_type_param, gen_packable_struct_sum_constraint};

pub fn impl_packable_struct(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ty_read_write = gen_type_param();
    let generics = &ast.generics;
    let constraints = gen_packable_struct_sum_constraint(generics);

    match &ast.data {
        syn::Data::Struct(ds) => {
            match &ds.fields {
                syn::Fields::Unnamed(fs) => {
                    if fs.unnamed.len() > 0x0F {
                        panic!("PackStream only supports structures with at most '{}' fields.", 0x0F);
                    }
                    let fields = fs.unnamed.len();

                    let mut res_pack = proc_macro2::TokenStream::new();
                    let mut res_unpack = proc_macro2::TokenStream::new();

                    for (i, f) in fs.unnamed.iter().enumerate() {
                        let f_ty = &f.ty;
                        res_pack.extend(quote! {
                            written += <f_ty as Pack<#ty_read_write >>::encode(&self.#i, writer)?;
                        });

                        res_unpack.extend(quote! {
                            <#f_ty as Unpack<#ty_read_write >>::decode(reader)?,
                        });
                    }

                    quote! {
                        impl<#constraints> PackableStruct for #name #generics {
                        fn write_structure_body<#ty_read_write: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
                            let mut written = 0;
                            #res_pack
                            Ok(written)
                        }

                        fn read_structure_body<#ty_read_write: Read>(reader: &mut T) -> Result<Self, DecodeError> {
                            Ok(#name(#res_unpack))
                        }
                        const FIELDS: usize = #fields;
                        }
                    }
                },

                syn::Fields::Named(fs) => {
                    if fs.named.len() > 0x0F {
                        panic!("PackStream only supports structures with at most '{}' fields.", 0x0F);
                    }
                    let fields = fs.named.len();

                    let mut res_pack = proc_macro2::TokenStream::new();
                    let mut res_unpack = proc_macro2::TokenStream::new();

                    for f in &fs.named {
                        let f_ident = (&f.ident).as_ref().unwrap();
                        let f_ty = &f.ty;
                        res_pack.extend(quote! {
                            written += <#f_ty as Pack<#ty_read_write>>::encode(&self.#f_ident, writer)?;
                        });
                        res_unpack.extend(quote! {
                            #f_ident: <#f_ty as Unpack<#ty_read_write>>::decode(reader)?,
                        });
                    }

                    quote! {
                        impl<#constraints> PackableStruct for #name #generics {
                        const FIELDS: usize = #fields;
                        fn write_structure_body<#ty_read_write: Write>(&self, writer: &mut #ty_read_write) -> Result<usize, EncodeError>{
                            let mut written = 0;
                            #res_pack
                            Ok(written)
                        }

                        fn read_structure_body<#ty_read_write: Read>(reader: &mut #ty_read_write) -> Result<Self, DecodeError> {
                            Ok(
                            #name {
                                #res_unpack
                            })
                        }
                        }
                    }
                },

                syn::Fields::Unit => {
                    quote! {
                        impl<#constraints> PackableStruct for #name #generics {
                            fn write_structure_body<#ty_read_write: Write>(&self, writer: &mut #ty_read_write) -> Result<usize, EncodeError> {
                                Ok(0)
                            }

                            fn read_structure_body<#ty_read_write: Read>(reader: &mut #ty_read_write) -> Result<Self, DecodeError> {
                                Ok(#name {})
                            }
                        }
                    }
                }
            }
        },

        _ => panic!("Cannot derive PackableStruct for anything besides struct."),
    }
}
