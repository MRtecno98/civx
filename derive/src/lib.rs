use std::collections::HashSet;

use quote::{format_ident, quote};
use syn::{Ident, ItemStruct, Token, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated};

struct Args {
	flags: Punctuated<Ident, Token![,]>,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let flags = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
		Ok(Args { flags })
	}
}

#[proc_macro_attribute]
pub fn civx(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let flags = parse_macro_input!(attr as Args)
		.flags.into_iter().collect::<HashSet<_>>();

	let _clap = flags.contains(&format_ident!("clap"));

	let mut item_struct = parse_macro_input!(item as ItemStruct);

	#[cfg(feature = "clap")]
	if _clap && let Some(derive) 
		= item_struct.attrs.iter_mut().find(|a| a.path().is_ident("derive")) {
		use syn::{Meta, MetaList};

		if let Meta::List(MetaList { ref mut tokens, .. }) = derive.meta {
			tokens.extend(quote! {, clap::Args});
		}
	}

	for _field in item_struct.fields.iter_mut() {
		#[cfg(feature = "clap")]
		if _clap {
			if _field.attrs.contains(&syn::parse_quote!(#[serde(skip)])) {
				_field.attrs.push(syn::parse_quote!(#[clap(skip)]));
			} else if _field.attrs.contains(&syn::parse_quote!(#[serde(flatten)])) {
				_field.attrs.push(syn::parse_quote!(#[clap(flatten)]));
			} else {
				if _field.ty == syn::parse_quote!(Option<bool>) {
					_field.attrs.push(syn::parse_quote!(
						#[arg(
							long, 
							action = clap::ArgAction::Set, 
							value_parser = clap::builder::BoolishValueParser::new()
						)]
					));
				} else {
					_field.attrs.push(syn::parse_quote!(#[arg(long)]));
				}
			}
		}
	}

	quote! {
		#item_struct
	}.into()
}
