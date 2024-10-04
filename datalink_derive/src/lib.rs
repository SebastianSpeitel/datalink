use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod internals;
use internals::{is_new_type, Attrs, Field};

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
                    self.0.query(request);
                });
                this.query_owned.extend(quote! {
                    self.0.query_owned(request);
                });
            }
            syn::Data::Struct(ref data) => {
                for (idx, f) in data.fields.iter().enumerate() {
                    let field = Field::new(f, idx)?;
                    this.field(field);
                }
            }
            syn::Data::Enum(ref data) => {
                for variant in &data.variants {
                    this.variant(variant);
                }
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

    fn field(&mut self, field: Field) {
        self.query.extend(field.query());
        self.query_owned.extend(field.query_owned());
    }

    fn variant(&mut self, variant: &syn::Variant) {}
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
                fn query(&self, request: &mut impl ::datalink::Request) {
                    #query
                }

                #[inline]
                fn query_owned(self, request: &mut impl ::datalink::Request) where Self: Sized {
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
