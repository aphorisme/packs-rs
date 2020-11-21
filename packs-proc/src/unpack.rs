use proc_macro2::{Ident, TokenStream};
use syn::{Generics, Attribute, DataStruct};
use crate::common::enums::Tags;
use crate::common::{get_fields_attr, get_unpack_attr_param, get_tag_attr, gen_type_param, get_singleton_field_type};
use quote::quote;

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
        impl #generics Unpack for #ident #generics {
            fn decode_body<#ty_param: std::io::Read>(marker: Marker, reader: &mut #ty_param) -> Result<Self, DecodeError> {
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

pub fn impl_unpack_struct(ident: &Ident, generics: &Generics, attrs: &[Attribute], s: &DataStruct) -> TokenStream {
    let tag = get_tag_attr(attrs).expect("Need #[tag = u8] attribute on struct.");
    let ty_read = gen_type_param();

    let mut unpack_cases = proc_macro2::TokenStream::new();
    let mut struct_build = proc_macro2::TokenStream::new();
    let mut fields_len = 0;

    for f in &s.fields {
        let f_ty = &f.ty;
        let f_ident = f.ident.as_ref().expect("Expected field ident");
        fields_len += get_fields_attr(&f.attrs).unwrap_or(1);

        let unpack =
            // use #[unpack(func)]:
            if let Some(func) = get_unpack_attr_param(&f.attrs) {
                quote! {
                    let #f_ident = #func(reader)?;
                }
            } else {
                quote! {
                    let #f_ident = <#f_ty as Unpack>::decode(reader)?;
                }
            };

        unpack_cases.extend(unpack);

        struct_build.extend(
            quote! {
                #f_ident,
            }
        );
    }

    if fields_len > 15 {
        panic!("More then 15 fields are not allowed for a struct.");
    }

    quote! {
        impl #generics Unpack for #ident #generics {
            fn decode_body<#ty_read: std::io::Read>(marker: Marker, reader: &mut #ty_read) -> Result<Self, DecodeError> {
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
