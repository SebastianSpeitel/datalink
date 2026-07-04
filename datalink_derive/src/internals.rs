use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Field<'a> {
    field: &'a syn::Field,
    attrs: Attrs,
    member: syn::Member,
}

impl<'a> Field<'a> {
    #[inline]
    pub fn new(field: &'a syn::Field, index: usize) -> syn::Result<Self> {
        let member = field
            .ident
            .as_ref()
            .map(|i| i.to_owned().into())
            .unwrap_or_else(|| index.into());

        let attrs = field
            .attrs
            .iter()
            .find_map(|attr| {
                if attr.path().is_ident("data") {
                    Some(Attrs::from_attr(attr))
                } else {
                    None
                }
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            field,
            attrs,
            member,
        })
    }

    fn link_key(&self) -> Option<TokenStream> {
        self.field
            .ident
            .as_ref()
            .map(|ident| ident.to_string().into_token_stream())
    }

    pub fn query<const O: bool>(&self) -> TokenStream {
        let member = &self.member;
        if O {
            self.query_owned(quote!(self.#member))
        } else {
            self.query_by_ref(quote!(self.#member))
        }
    }

    pub fn query_by_ref(&self, expr: TokenStream) -> TokenStream {
        let mut query = TokenStream::new();

        let mut link = match self.attrs.link {
            Mode::Skip => None,
            Mode::Auto if self.attrs.provide == Mode::Copy => Some(quote!(#expr)),
            Mode::Copy => Some(quote!(::datalink::LinkBuilder::new(#expr))),
            Mode::Auto | Mode::Ref => {
                Some(quote!(::datalink::LinkBuilder::new_ownable(&#expr)))
            }
            Mode::Clone => Some(quote!(::datalink::LinkBuilder::new_ownable(&#expr))),
            Mode::ToOwned => Some(quote!(::datalink::LinkBuilder::new_ownable(&#expr))),
        };

        let value = match self.attrs.provide {
            Mode::Skip => None,
            Mode::Auto if self.attrs.link == Mode::Copy => Some(quote!(
                request.provide_value(#expr);
            )),
            Mode::Auto | Mode::Ref => Some(quote!(
                request.provide_ref(&#expr);
            )),
            Mode::Clone => Some(quote!(
                request.provide_value_with(|| (#expr).clone());
            )),
            Mode::ToOwned => Some(quote!(
                request.provide_value_with(|| (#expr).to_owned());
            )),
            Mode::Copy => Some(quote!(
                request.provide_value(#expr);
            )),
        };

        if let Some(link) = &mut link {
            if let Some(key) = self.link_key() {
                link.extend(quote!(.key(#key)));
            }

            query.extend(quote! {
                request.provide_link_with(|| #link);
            });
        }

        query.extend(value);
        query
    }

    pub fn query_owned(&self, expr: TokenStream) -> TokenStream {
        let mut query = TokenStream::new();

        let double_usage = match (self.attrs.provide, self.attrs.link) {
            (Mode::Skip | Mode::Copy, ..) | (.., Mode::Skip | Mode::Copy) => false,
            _ => true,
        };

        let mut link = match self.attrs.link {
            Mode::Skip => None,
            Mode::Ref if double_usage => Some(quote! {
                ::datalink::LinkBuilder::new_ownable(&#expr)
            }),
            Mode::Clone if double_usage => Some(quote! {
                ::datalink::LinkBuilder::new_ownable(&#expr)
            }),
            Mode::ToOwned if double_usage => Some(quote! {
                ::datalink::LinkBuilder::new_ownable(&#expr)
            }),
            _ => Some(quote!(::datalink::LinkBuilder::new(#expr))),
        };

        let provide = match self.attrs.provide {
            Mode::Skip => None,
            _ => Some(quote!(request.provide_value(#expr);)),
        };

        if let Some(link) = &mut link {
            if let Some(key) = self.link_key() {
                link.extend(quote!(.key(#key)));
            }

            query.extend(quote! {
                request.provide_link_with(|| #link);
            });
        }

        query.extend(provide);
        query
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Skip,
    Auto,
    Ref,
    Clone,
    ToOwned,
    Copy,
}

impl syn::parse::Parse for Mode {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        match ident.to_string().as_str() {
            "skip" => Ok(Self::Skip),
            "ref" => Ok(Self::Ref),
            "clone" => Ok(Self::Clone),
            "to_owned" => Ok(Self::ToOwned),
            "copy" => Ok(Self::Copy),
            "auto" => Ok(Self::Auto),
            _ => Err(syn::Error::new_spanned(
                ident,
                "expected one of: skip, ref, clone, to_owned, copy",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attrs {
    provide: Mode,
    link: Mode,
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            provide: Mode::Skip,
            link: Mode::Auto,
        }
    }
}

impl Attrs {
    pub fn from_attr(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut provide = Mode::Skip;
        let mut link = Mode::Auto;
        let mut both = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                both.replace(Mode::Skip);
                return Ok(());
            }
            if meta.path.is_ident("provide") {
                if let Ok(mode) = meta.value() {
                    provide = mode.parse()?;
                } else {
                    provide = Mode::Auto;
                }
                return Ok(());
            }
            if meta.path.is_ident("link") {
                if let Ok(mode) = meta.value() {
                    link = mode.parse()?;
                } else {
                    link = Mode::Auto;
                }
                return Ok(());
            }
            if meta.path.is_ident("clone") {
                both.replace(Mode::Clone);
                return Ok(());
            }
            if meta.path.is_ident("to_owned") {
                both.replace(Mode::ToOwned);
                return Ok(());
            }
            if meta.path.is_ident("copy") {
                both.replace(Mode::Copy);
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;

        if let (Some(mode), Mode::Auto) = (both, provide) {
            provide = mode;
        }

        if let (Some(mode), Mode::Auto) = (both, link) {
            link = mode;
        }

        Ok(Self { provide, link })
    }
}

pub fn is_new_type(fields: &syn::DataStruct) -> bool {
    fields.fields.len() == 1 && matches!(fields.fields, syn::Fields::Unnamed(_))
}
