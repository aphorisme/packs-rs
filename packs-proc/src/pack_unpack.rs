use proc_macro2::{TokenStream};
use crate::common::{get_tag_attr, gen_type_param, gen_packable_struct_sum_constraint, get_singleton_field_type};
use quote::{quote};
use proc_macro2::Ident;
use syn::{Generics, Attribute};
use crate::common::enums::Tags;

pub fn impl_pack_struct(ident: &Ident, generics: &Generics, attrs: &Vec<Attribute>) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_write = gen_type_param();
    let constraint_sum = gen_packable_struct_sum_constraint(generics);

    quote! {
        impl<#ty_write: Write, #constraint_sum> Pack<#ty_write> for #ident #generics {
            fn encode(&self, writer: &mut #ty_write) -> Result<usize, EncodeError> {
                encode_struct(self, #tag, writer)
            }
        }
    }
}

pub fn impl_pack_sum(ident: &Ident, generics: &Generics, ast: &syn::DataEnum) -> TokenStream {
    let mut pack_cases = proc_macro2::TokenStream::new();
    let mut tags = Tags::with_capacity(ast.variants.len());

    for v in ast.variants.iter() {
        tags.add_from_attr(&v.attrs);
        let tag = tags.last_tag();

        let var_name = &v.ident;
        let var_type = get_singleton_field_type(v);

        pack_cases.extend(quote! {
            #ident::#var_name(v) => {
                Marker::Structure(<#var_type as PackableStruct>::FIELDS).encode(writer)?;
                writer.write_all(&[#tag])?;
                let mut written = <#var_type as PackableStruct>::write_structure_body(v, writer)?;
                Ok(2 + written)
            },
        });
    }

    let ty_param = gen_type_param();

    quote! {
        impl<#ty_param: Write, #generics> Pack<#ty_param> for #ident #generics  {
            fn encode(&self, writer: &mut #ty_param) -> Result<usize, EncodeError> {
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

    for v in ast.variants.iter() {
        tags.add_from_attr(&v.attrs);
        let tag = tags.last_tag();
        
        let var_name = &v.ident;
        let var_type = get_singleton_field_type(v);

       unpack_cases.extend(quote! {
            #tag => Ok(#ident::#var_name(<#var_type as PackableStruct>::read_structure_body(reader)?)),
        })
    }

    let ty_param = gen_type_param();

    quote! {
        impl<#ty_param: Read, #generics> Unpack<#ty_param> for #ident #generics {
            fn decode(reader: &mut #ty_param) -> Result<Self, DecodeError> {
                let marker = Marker::decode(reader)?;
                match marker {
                    Marker::Structure(_) => {
                        let mut buf = [0; 1];
                        reader.read_exact(&mut buf)?;
                        match buf[0] {
                            #unpack_cases
                            _ => Err(DecodeError::UnknownMarkerByte(buf[0])),
                        }
                    },
                    _ => Err(DecodeError::UnexpectedMarker(marker)),
                }
            }
        }
    }
}

pub fn impl_unpack_struct(ident: &Ident, generics: &Generics, attrs: &Vec<Attribute>) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_read = gen_type_param();
    let constraint_sum = gen_packable_struct_sum_constraint(generics);
    quote! {
        impl<#ty_read: Read, #constraint_sum> Unpack<#ty_read> for #ident #generics {
            fn decode(reader: &mut #ty_read) -> Result<Self, DecodeError> {
                decode_struct(#tag, reader)
            }
        }
    }
}
