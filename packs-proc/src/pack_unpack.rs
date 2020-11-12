use proc_macro2::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Attribute, DataStruct, Generics};

use crate::common::{gen_type_param, get_singleton_field_type, get_tag_attr};
use crate::common::enums::Tags;

pub fn impl_pack_struct(ident: &Ident, generics: &Generics, attrs: &Vec<Attribute>, s: &DataStruct) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_write = gen_type_param();

    let mut pack_cases = proc_macro2::TokenStream::new();
    let fields_len = s.fields.len();
    if fields_len > 15 {
        panic!("Only structs with at most 15 fields are supported.");
    }

    for f in &s.fields {
        let field_ident = f.ident.as_ref().expect("Expected identifier at field.");
        pack_cases.extend(
            quote! {
                written += self.#field_ident.encode(writer)?;
            }
        );
    }

    quote! {
        impl<#generics> Pack for #ident #generics {
            fn encode<#ty_write: Write>(&self, writer: &mut #ty_write) -> Result<usize, EncodeError> {
                let mut written =
                    Marker::Structure(#fields_len, #tag).encode(writer)?;
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
        impl<#generics> Pack for #ident #generics  {
            fn encode<#ty_param: Write>(&self, writer: &mut #ty_param) -> Result<usize, EncodeError> {
                match self {
                    #pack_cases
                }
            }
        }
    }
}

pub fn impl_unpack_sum(ident: &Ident, generics: &Generics, ast: &syn::DataEnum) -> TokenStream {
    let mut unpack_cases = proc_macro2::TokenStream::new();
    let mut tags = Tags::with_capacity(ast.variants.len());

    let ty_param = gen_type_param();

    for v in ast.variants.iter() {
        tags.add_from_attr(&v.attrs);
        let tag = tags.last_tag();

        let var_name = &v.ident;
        let var_type = get_singleton_field_type(v);

        unpack_cases.extend(quote! {
            #tag => Ok(#ident::#var_name(<#var_type as Unpack>::decode_body(marker, reader)?)),
        })
    }


    quote! {
        impl<#generics> Unpack for #ident #generics {
            fn decode_body<#ty_param: Read>(marker: Marker, reader: &mut #ty_param) -> Result<Self, DecodeError> {
                match marker {
                    Marker::Structure(_, tag) => {
                        match tag {
                            #unpack_cases
                            _ => Err(DecodeError::UnexpectedTagByte(tag)),
                        }
                    },
                    _ => Err(DecodeError::UnexpectedMarker(marker)),
                }
            }
        }
    }
}

pub fn impl_unpack_struct(ident: &Ident, generics: &Generics, attrs: &Vec<Attribute>, s: &DataStruct) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_read = gen_type_param();

    let mut unpack_cases = proc_macro2::TokenStream::new();
    let mut struct_build = proc_macro2::TokenStream::new();
    let fields_len = s.fields.len();

    for f in &s.fields {
        let f_ty = &f.ty;
        let f_ident = f.ident.as_ref().expect("Expected field ident");

        unpack_cases.extend(
            quote! {
                let #f_ident = <#f_ty as Unpack>::decode(reader)?;
            }
        );

        struct_build.extend(
            quote! {
                #f_ident,
            }
        );
    }

    quote! {
        impl<#generics> Unpack for #ident #generics {
            fn decode_body<#ty_read: Read>(marker: Marker, reader: &mut #ty_read) -> Result<Self, DecodeError> {
                match marker {
                    Marker::Structure(u, tag) => {
                        if #fields_len != u {
                            return Err(DecodeError::UnexpectedNumberOfFields(#fields_len, u))
                        }

                        if #tag != tag {
                            return Err(DecodeError::UnexpectedTagByte(tag))
                        }

                        #unpack_cases

                        Ok(#ident {
                            #struct_build
                        })
                    },
                    _ => Err(DecodeError::UnexpectedMarker(marker))
                }
            }
        }
    }
}
