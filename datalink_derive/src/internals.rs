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
        if let Some(ident) = &self.field.ident {
            Some(ident.to_string().into_token_stream())
        } else {
            None
        }
    }

    pub fn query(&self) -> TokenStream {
        let member = &self.member;

        let target = match self.attrs.link {
            Mode::Skip => None,
            _ if self.attrs.provide == Mode::Copy => Some(quote!(self.#member)),
            Mode::Copy => Some(quote!(self.#member)),
            Mode::Yes | Mode::Ref => Some(quote!(&self.#member)),
            Mode::Clone => Some(quote!(self.#member.clone())),
            Mode::ToOwned => Some(quote!(self.#member.to_owned())),
        };

        let key = self.link_key();

        let provide = match self.attrs.provide {
            Mode::Skip => None,
            _ if self.attrs.link == Mode::Copy => Some(quote!(request.provide(self.#member);)),
            Mode::Yes | Mode::Ref => Some(quote!(request.provide_ref(&self.#member);)),
            Mode::Clone => Some(quote!(request.provide(self.#member.clone());)),
            Mode::ToOwned => Some(quote!(request.provide(self.#member.to_owned());)),
            Mode::Copy => Some(quote!(request.provide(self.#member);)),
        };

        let mut query = match (target, key) {
            (None, ..) => quote!(),
            (Some(target), Some(key)) => quote! {
                request.push_link((#key, #target));
            },
            (Some(target), None) => quote! {
                request.push_link(::datalink::link::Unkeyed(#target));
            },
        };

        query.extend(provide);
        query
    }

    pub fn query_owned(&self) -> TokenStream {
        let member = &self.member;

        let double_usage = match (self.attrs.provide, self.attrs.link) {
            (Mode::Skip | Mode::Copy, ..) | (.., Mode::Skip | Mode::Copy) => false,
            _ => true,
        };

        let target = match self.attrs.link {
            Mode::Skip => None,
            Mode::Ref if double_usage => Some(quote!(&self.#member)),
            Mode::Clone if double_usage => Some(quote!(self.#member.clone())),
            Mode::ToOwned if double_usage => Some(quote!(self.#member.to_owned())),
            _ => Some(quote!(self.#member)),
        };

        let key = self.link_key();

        let provide = match self.attrs.provide {
            Mode::Skip => None,
            _ => Some(quote!(request.provide(self.#member);)),
        };

        let mut query = match (target, key) {
            (None, ..) => quote!(),
            (Some(target), Some(key)) => quote! {
                request.push_link((#key, #target));
            },
            (Some(target), None) => quote! {
                request.push_link(::datalink::link::Unkeyed(#target));
            },
        };

        query.extend(provide);
        query
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Skip,
    Yes,
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
            "yes" => Ok(Self::Yes),
            "ref" => Ok(Self::Ref),
            "clone" => Ok(Self::Clone),
            "to_owned" => Ok(Self::ToOwned),
            "copy" => Ok(Self::Copy),
            _ => Err(syn::Error::new_spanned(
                ident,
                "expected one of: skip, yes, ref, clone, to_owned, copy",
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
            link: Mode::Yes,
        }
    }
}

impl Attrs {
    pub fn from_attr(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut attrs = Self::default();

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                attrs.provide = Mode::Skip;
                attrs.link = Mode::Skip;
                return Ok(());
            }
            if meta.path.is_ident("provide") {
                if let Ok(mode) = meta.value() {
                    attrs.provide = mode.parse()?;
                } else {
                    attrs.provide = Mode::Yes;
                }
                return Ok(());
            }
            if meta.path.is_ident("link") {
                if let Ok(mode) = meta.value() {
                    attrs.link = mode.parse()?;
                } else {
                    attrs.link = Mode::Yes;
                }
                return Ok(());
            }
            if meta.path.is_ident("clone") {
                attrs.link = Mode::Clone;
                return Ok(());
            }
            if meta.path.is_ident("to_owned") {
                attrs.link = Mode::ToOwned;
                return Ok(());
            }
            if meta.path.is_ident("copy") {
                attrs.link = Mode::Copy;
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;

        Ok(attrs)
    }
}

pub fn is_new_type(fields: &syn::DataStruct) -> bool {
    fields.fields.len() == 1 && matches!(fields.fields, syn::Fields::Unnamed(_))
}
