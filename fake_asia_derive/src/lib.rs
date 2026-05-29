use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(FakeAsia)]
pub fn derive_fake_asia(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_inits = fields.named.iter().map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        let field_type = &f.ty;
                        let type_str = quote!(#field_type).to_string();

                        let expr = match field_name.to_string().as_str() {
                            "name" => quote!(fake_asia::chinese_name(rng)),
                            "phone" => quote!(fake_asia::chinese_phone_number(rng)),
                            "id_card" => quote!(fake_asia::chinese_id_card(rng)),
                            "email" => quote!(fake_asia::email(rng)),
                            "address" => {
                                if type_str.contains("Japanese") {
                                    quote!(fake_asia::japanese_address(rng))
                                } else if type_str.contains("Korean") {
                                    quote!(fake_asia::korean_address(rng))
                                } else if type_str.contains("Indian") {
                                    quote!(fake_asia::indian_address(rng))
                                } else {
                                    quote!(fake_asia::chinese_address(rng))
                                }
                            }
                            "company" => {
                                if type_str.contains("Japanese") {
                                    quote!(fake_asia::japanese_company(rng))
                                } else {
                                    quote!(fake_asia::chinese_company(rng))
                                }
                            }
                            _ => {
                                return quote! {
                                    compile_error!(concat!("FakeAsia derive: unsupported field type for field `", stringify!(#field_name), "`. Consider implementing FakeAsia manually."));
                                };
                            }
                        };

                        quote! {
                            #field_name: #expr
                        }
                    });

                    quote! {
                        let mut rng = rand::thread_rng();
                        Self {
                            #(#field_inits,)*
                        }
                    }
                }
                _ => {
                    return quote! {
                        compile_error!("FakeAsia derive: only named structs are supported");
                    }.into();
                }
            }
        }
        _ => {
            return quote! {
                compile_error!("FakeAsia derive: only structs are supported");
            }.into();
        }
    };

    let expanded = quote! {
        impl #impl_generics fake_asia::FakeAsia for #name #ty_generics #where_clause {
            fn fake_asia<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                #body
            }
        }
    };

    TokenStream::from(expanded)
}
