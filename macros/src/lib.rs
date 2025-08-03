use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(CommandCategory)]
pub fn derive_command_category(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let Data::Enum(data) = input.data else {
		panic!("can only derive `CommandCategory` for enum type.");
	};

	let ident = format_ident!("{}", input.ident);

	let variants = data.variants.iter().map(|variant| {
		let ident = format_ident!("{}", variant.ident);
		let name = variant.ident.to_string().to_case(Case::Kebab);

		quote! { #name => Ok(Self::#ident(rest.parse()?)), }
	});

	let expanded = quote! {
		impl std::str::FromStr for #ident {
			type Err = ::parser::ParseCommandError;

			fn from_str(string: &str) -> Result<Self, Self::Err> {
				use ::parser::*;

				let (category, rest) = match string.trim().split_once(' ') {
					Some((category, rest)) => (category, rest),
					None => (string.trim(), ""),
				};

				match category {
					"" => Err(ParseCommandError::EmptyString),
					#(#variants)*
					_ => Err(ParseCommandError::UnknownCategory(category.into())),
				}
			}
		}
	};

	expanded.into()
}

#[proc_macro_derive(Subcommand, attributes(fallback_to_default))]
pub fn derive_subcommand(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let Data::Enum(data) = input.data else {
		panic!("can only derive `Subcommand` for enum type.");
	};

	let ident = format_ident!("{}", input.ident);

	let variants = data.variants.iter().map(|variant| {
		let ident = format_ident!("{}", variant.ident);
		let name = variant.ident.to_string().to_case(Case::Kebab);

		let constructor = if variant.fields.is_empty() {
			quote! { Self::#ident }
		} else {
			let field_initializers = variant.fields.iter().filter_map(|field| {
				let Some(field_ident) =
					field.ident.as_ref().map(|ident| format_ident!("{}", ident))
				else {
					return None;
				};

				let field_name = field_ident.to_string().to_case(Case::Kebab);
				let field_type = field.ty.to_token_stream().to_string();
				let is_optional = field_type.starts_with("Option <");

				let field_attrs = field
					.attrs
					.iter()
					.map(|attr| attr.meta.to_token_stream().to_string())
					.collect::<Vec<_>>();

				if is_optional {
					Some(quote! { #field_ident: args.get_optional(#field_name)?, })
				} else {
					if field_attrs.contains(&"fallback_to_default".to_string()) {
						Some(quote! {
							#field_ident: match args.get(#field_name) {
								Ok(result) => Ok(result),
								Err(ParseCommandError::MissingArgument(_)) => Ok(Default::default()),
								error => error,
							}?,
						})
					} else {
						Some(quote! { #field_ident: args.get(#field_name)?, })
					}
				}
			});

			quote! { Self::#ident { #(#field_initializers)* } }
		};

		let field_strings = variant
			.fields
			.iter()
			.filter_map(|field| field.ident.as_ref().map(|ident| format_ident!("{}", ident)))
			.map(|field_name| field_name.to_string().to_case(Case::Kebab));

		quote! {
			#name => {
				let args = Arguments::parse(rest, &[#(#field_strings),*])?;
				Ok(#constructor)
			}
		}
	});

	let expanded = quote! {
		impl std::str::FromStr for #ident {
			type Err = ::parser::ParseCommandError;

			fn from_str(string: &str) -> Result<Self, Self::Err> {
				use ::parser::*;

				let (subcommand, rest) = match string.split_once(' ') {
					Some((subcommand, rest)) => (subcommand, rest),
					None => (string, ""),
				};

				match subcommand {
					"" => Err(ParseCommandError::NoSubcommand),
					#(#variants)*
					_ => Err(ParseCommandError::UnknownSubcommand(subcommand.to_string())),
				}
			}
		}
	};

	expanded.into()
}
