use syn::export::TokenStream;
use quote::quote;
use crate::common::{get_tag_attr, gen_packable_struct_sum_constraint, gen_type_param};

pub fn impl_packable_struct_sum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let ty_sum = gen_packable_struct_sum_constraint(generics);
    let ty_read_write = gen_type_param();
    match &ast.data {
        syn::Data::Enum(e) => {
            let mut tags = Vec::with_capacity(e.variants.len());
            let mut last_tag = 0x00u8;
            let mut pack = proc_macro2::TokenStream::new();
            let mut field_lens = proc_macro2::TokenStream::new();
            let mut get_tag_byte = proc_macro2::TokenStream::new();
            let mut unpack = proc_macro2::TokenStream::new();
            for v in e.variants.iter() {
                // look for tag:
                let tag =
                    if let Some(val) = get_tag_attr(&v.attrs) {
                        last_tag = val;
                        val
                    } else {
                        last_tag += 1;
                        last_tag
                    };
                if tags.contains(&tag) {
                    panic!("Tag '{:X}' is not unique!", tag);
                }

                tags.push(tag);

                let ident = &v.ident;
                let ty = if v.fields.len() != 1 {
                    panic!("Variant '{}' has != 1 fields.", ident)
                } else {
                    &v.fields.iter().next().unwrap().ty
                };

                pack.extend(quote! {
                    #name::#ident(ref p) => <#ty as PackableStruct>::write_structure_body(p, writer),
                });

                field_lens.extend(quote! {
                    #name::#ident(_) => <#ty as PackableStruct>::FIELDS,
                });

                get_tag_byte.extend(quote! {
                    #name::#ident(_) => #tag,
                });

                unpack.extend(quote! {
                    if tag_byte == #tag {
                        if size != <#ty as PackableStruct>::FIELDS {
                            return Err(DecodeError::UnexpectedNumberOfFields(<#ty as PackableStruct>::FIELDS, size));
                        }
                        return Ok(#name::#ident(<#ty as PackableStruct>::read_structure_body(reader)?));
                    }
                });
            }

            let gen = quote! {
                impl<#ty_sum> PackableStructSum for #name #generics {
                    fn read_struct_body<#ty_read_write: Read>(size: usize, tag_byte: u8, reader: &mut #ty_read_write) -> Result<Self, DecodeError> {
                        #unpack
                        Err(DecodeError::UnexpectedTagByte(tag_byte))
                    }

                    fn write_struct_body<#ty_read_write: Write>(&self, writer: &mut #ty_read_write) -> Result<usize, EncodeError> {
                        match self {
                            #pack
                        }
                    }

                    fn fields_len(&self) -> usize {
                        match self {
                            #field_lens
                        }
                    }

                    fn tag_byte(&self) -> u8 {
                        match self {
                            #get_tag_byte
                        }
                    }
                }
            };

            gen.into()
        },

        _ => panic!("Currently only enums are supported as struct sums.")
    }
}
