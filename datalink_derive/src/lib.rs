use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Variant};

mod internals;
use internals::{is_new_type, Field};

#[proc_macro_derive(Data, attributes(data))]
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    let data_impl = match DataImpl::from_ast(&input) {
        Ok(data_impl) => data_impl,
        Err(e) => return e.to_compile_error().into(),
    };

    data_impl.into()
}

struct DataImpl<'a> {
    input: &'a syn::DeriveInput,
    query: TokenStream,
    query_owned: TokenStream,
}

impl<'a> DataImpl<'a> {
    pub fn from_ast(input: &'a syn::DeriveInput) -> Result<Self, syn::Error> {
        let mut this = Self {
            input,
            query: TokenStream::new(),
            query_owned: TokenStream::new(),
        };

        match input.data {
            syn::Data::Struct(ref data) if data.fields.is_empty() => {
                // Unit Struct
            }
            syn::Data::Struct(ref data) if is_new_type(data) => {
                this.query.extend(quote! {
                    ::datalink::Data::query(&self.0, request);
                });
                this.query_owned.extend(quote! {
                    ::datalink::Data::query_owned(self.0, request);
                });
            }
            syn::Data::Struct(ref data) => {
                let fields = data
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| Field::new(f, i));
                for field in fields {
                    let field = field?;
                    this.query.extend(field.query::<false>());
                    this.query_owned.extend(field.query::<true>());
                }
            }
            syn::Data::Enum(ref data) => {
                let by_ref = data.variants.iter().map(Self::variant::<false>);
                let owned = data.variants.iter().map(Self::variant::<true>);

                this.query.extend(quote! {
                    match self {
                        #(#by_ref)*
                    }
                });
                this.query_owned.extend(quote! {
                    match self {
                        #(#owned)*
                    }
                });
            }
            syn::Data::Union(_) => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Data cannot be derived for unions",
                ));
            }
        }

        Ok(this)
    }

    fn variant<const O: bool>(variant: &syn::Variant) -> TokenStream {
        let Variant {
            attrs: _,
            ident,
            fields,
            discriminant,
        } = variant;

        let mut arm = TokenStream::new();

        if let Some((_, e)) = discriminant {
            arm.extend(quote! {
                request.provide_link(("discriminant", #e));
            });
        }

        let (pattern, field_queries) = match fields {
            syn::Fields::Unit => (quote!(), TokenStream::new()),
            syn::Fields::Unnamed(fields) => {
                let mut pattern_bindings = Vec::new();
                let mut queries = TokenStream::new();
                for (i, f) in fields.unnamed.iter().enumerate() {
                    let var_name = quote::format_ident!("_{}", i);
                    let field = match Field::new(f, i) {
                        Ok(field) => field,
                        Err(e) => {
                            queries.extend(e.to_compile_error());
                            continue;
                        }
                    };
                    if O {
                        pattern_bindings.push(quote!(#var_name));
                        queries.extend(field.query_owned(quote!(#var_name)));
                    } else {
                        pattern_bindings.push(quote!(ref #var_name));
                        queries.extend(field.query_by_ref(quote!(*#var_name)));
                    }
                }
                (quote!( ( #(#pattern_bindings),* ) ), queries)
            }
            syn::Fields::Named(fields) => {
                let mut pattern_bindings = Vec::new();
                let mut queries = TokenStream::new();
                for (i, f) in fields.named.iter().enumerate() {
                    let field_name = f.ident.as_ref().unwrap();
                    let field = match Field::new(f, i) {
                        Ok(field) => field,
                        Err(e) => {
                            queries.extend(e.to_compile_error());
                            continue;
                        }
                    };
                    if O {
                        pattern_bindings.push(quote!(#field_name));
                        queries.extend(field.query_owned(quote!(#field_name)));
                    } else {
                        pattern_bindings.push(quote!(ref #field_name));
                        queries.extend(field.query_by_ref(quote!(*#field_name)));
                    }
                }
                (quote!( { #(#pattern_bindings),* } ), queries)
            }
        };

        arm.extend(field_queries);

        quote!(Self::#ident #pattern => {#arm})
    }
}

impl ToTokens for DataImpl<'_> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input,
            query,
            query_owned,
        } = self;

        let ident = &input.ident;

        let data_impl = quote! {
            #[automatically_derived]
            impl ::datalink::Data for #ident {
                #[inline]
                fn query(&self, mut request: impl ::datalink::Request) {
                    use ::datalink::RequestExt as _;
                    #query
                }

                #[inline]
                fn query_owned(self, mut request: impl ::datalink::Request) where Self: Sized {
                    use ::datalink::RequestExt as _;
                    #query_owned
                }
            }
        };
        tokens.extend(data_impl);
    }
}

impl Into<proc_macro::TokenStream> for DataImpl<'_> {
    #[inline]
    fn into(self) -> proc_macro::TokenStream {
        self.into_token_stream().into()
    }
}
