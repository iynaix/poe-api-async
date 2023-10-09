use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident};

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(gql))]
struct GQLField {
    #[deluxe(default)]
    r#where: bool,
    #[deluxe(default)]
    orderby: bool,
}

fn syn_type_to_idents(ty: &syn::Type) -> Vec<String> {
    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            let mut acc = Vec::new();
            for segment in &path.segments {
                let ident = &segment.ident;
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                        if let Some(syn::GenericArgument::Type(ty)) = args.args.iter().next() {
                            // add the current ident
                            acc.push(ident.to_string());
                            // recurse
                            acc.extend(syn_type_to_idents(ty));
                        }
                    }
                    _ => {
                        acc.push(ident.to_string());
                    }
                }
            }
            acc
        }
        _ => Vec::new(),
    }
}

struct FieldInfo {
    name: String,
    ty: Vec<String>,
    attrs: GQLField,
}

impl FieldInfo {
    fn from_ast(ast: &mut DeriveInput) -> deluxe::Result<Vec<FieldInfo>> {
        let mut field_info = Vec::new();

        if let syn::Data::Struct(data) = &mut ast.data {
            for field in &mut data.fields {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let attrs: GQLField = deluxe::extract_attributes(field)?;

                let ty_idents = syn_type_to_idents(&field.ty);
                field_info.push(Self {
                    name: field_name,
                    ty: ty_idents,
                    attrs,
                });
            }
        } else {
            panic!("Only structs with named fields are supported");
        }

        Ok(field_info)
    }

    fn where_struct_field(&self) -> Option<TokenStream> {
        if !self.attrs.r#where {
            return None;
        }

        // should be sufficient to handle the rightmost (innermost) type
        if let Some(ty) = self.ty.last() {
            let name = format_ident!("{}", &self.name);
            let filter_prefix = match ty.as_str() {
                "String" => "String",
                "i32" => "Int",
                "f64" => "Float",
                "bool" => "Boolean",
                _ => ty,
            };
            let filter_ident = format_ident!("{}Filter", filter_prefix);
            return Some(quote! { pub #name: Option<crate::schema::filters::#filter_ident>, });
        }

        None
    }

    fn whereinput_filter_if_let(&self) -> Option<TokenStream> {
        if !self.attrs.r#where {
            return None;
        }

        let key = format_ident!("{}", &self.name);
        let item_value = quote! { item.#key };

        let mut body = self
            .ty
            .iter()
            .rev()
            .fold(TokenStream::new(), |acc, ty| match ty.as_str() {
                "Vec" => {
                    quote! {
                        let filter_value = #item_value.to_owned();
                        if !#item_value.iter().any(|arr_item| {
                            let filter_value = arr_item.to_owned();
                            #acc
                            else {
                                return true;
                            }
                        }) {
                            return false;
                        }
                    }
                }
                "Option" => {
                    quote! {
                        if let Some(_) = #item_value {
                            let filter_value = #item_value.to_owned().unwrap();
                            #acc
                        }
                    }
                }
                _ => quote! {
                    if !filter_obj.filter_fn(filter_value) {
                        return false;
                    }
                },
            });

        // only a single token, filter value is the item directly
        if self.ty.len() == 1 {
            body = quote! {
                let filter_value = #item_value.to_owned();
                #body
            };
        }

        // finally add the outermost shell
        let ret = quote! {
            if let Self { #key: Some(filter_obj), .. } = self {
                #body
            }
        };

        Some(ret)
    }

    fn orderby_struct_field(&self) -> Option<TokenStream> {
        if !self.attrs.orderby {
            return None;
        }

        let name = format_ident!("{}", &self.name);
        Some(quote! { pub #name: Option<crate::schema::Orderby>, })
    }

    fn orderbyinput_from_orderpairs_match(&self) -> Option<TokenStream> {
        if !self.attrs.orderby {
            return None;
        }

        let name = format_ident!("{}", &self.name);
        Some(quote! {
            "#name" => Some(Self {
                #name: Some(orderby_value),
                ..Self::default()
            }),
        })
    }

    fn orderbyinput_cmp_orderby_match(&self) -> Option<TokenStream> {
        if !self.attrs.orderby {
            return None;
        }

        let name = format_ident!("{}", &self.name);

        Some(quote! {
            Self {
                #name: Some(v),
                ..
            } => match v {
                crate::schema::Orderby::Asc => a.#name.partial_cmp(&b.#name).unwrap(),
                crate::schema::Orderby::Desc => a.#name.partial_cmp(&b.#name).unwrap().reverse(),
            },
        })
    }
}

fn where_struct(fields: &[FieldInfo], model_ident: &Ident) -> TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let where_ident = format_ident!("{}Where", model_ident);
    let struct_fields: Vec<_> = fields
        .iter()
        .filter_map(|info| info.where_struct_field())
        .collect();

    quote! {
        #[derive(Debug, async_graphql::InputObject)]
        pub struct #where_ident {
            #(#struct_fields)*
            // recursive filters
            pub and: Option<Vec<#where_ident>>,
            pub or: Option<Vec<#where_ident>>,
            pub not: Option<Vec<#where_ident>>,
        }

    }
}

fn impl_whereinput(fields: &[FieldInfo], model_ident: &Ident) -> TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let where_ident = format_ident!("{}Where", model_ident);
    let whereinput_filter_if_let: Vec<_> = fields
        .iter()
        .filter_map(|info| info.whereinput_filter_if_let())
        .collect();

    quote! {
        impl crate::schema::filters::WhereInput for #where_ident {
            type Output = #model_ident;

            fn and(&self) -> Option<&Vec<#where_ident>> {
                self.and.as_ref()
            }

            fn or(&self) -> Option<&Vec<#where_ident>> {
                self.or.as_ref()
            }

            fn not(&self) -> Option<&Vec<#where_ident>> {
                self.not.as_ref()
            }

            fn filter(&self, items: Vec<Self::Output>) -> Vec<Self::Output> {
                items.into_iter().filter(|item| {
                    #(#whereinput_filter_if_let)*
                    return true;
                }).collect()
            }
        }
    }
}

fn orderby_struct(fields: &[FieldInfo], model_ident: &Ident) -> TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let orderby_ident = format_ident!("{}Orderby", model_ident);
    let struct_fields = fields
        .iter()
        .filter_map(|info| info.orderby_struct_field())
        .collect::<Vec<_>>();

    quote! {
        #[derive(Debug, Default, async_graphql::InputObject)]
        pub struct #orderby_ident {
            #(#struct_fields)*
        }
    }
}

fn impl_orderbyinput(fields: &[FieldInfo], model_ident: &Ident) -> TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let orderby_ident = format_ident!("{}Orderby", model_ident);
    let from_orderpairs_match = fields
        .iter()
        .filter_map(|info| info.orderbyinput_from_orderpairs_match())
        .collect::<Vec<_>>();

    let cmp_orderby_match = fields
        .iter()
        .filter_map(|info| info.orderbyinput_cmp_orderby_match())
        .collect::<Vec<_>>();

    quote! {
        impl crate::schema::orderby::OrderbyInput for #orderby_ident {
            type Output = #model_ident;

            fn from_orderbypairs(orderby_vec: Vec<crate::schema::orderby::OrderbyPair>) -> Vec<Self>
            where
                Self: Sized,
            {
                orderby_vec
                    .into_iter()
                    .filter_map(
                        |(orderby_name, orderby_value)| match orderby_name.as_str() {
                            // generate match statements for each field
                            #(#from_orderpairs_match)*
                            _ => None,
                        },
                    )
                    .collect()
            }

            fn cmp_orderby(&self, a: &Self::Output, b: &Self::Output) -> std::cmp::Ordering {
                match self {
                    // generate match statements for each field
                    #(#cmp_orderby_match)*
                    _ => panic!("Unreachable: empty orderby!"),
                }
            }
        }
    }
}

pub fn gqlmodel_core(item: TokenStream) -> deluxe::Result<TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item).unwrap();

    let model_ident = ast.ident.to_owned();
    let fields_info = FieldInfo::from_ast(&mut ast)?;

    let where_struct = where_struct(&fields_info, &model_ident);
    let impl_whereinput = impl_whereinput(&fields_info, &model_ident);
    let orderby_struct = orderby_struct(&fields_info, &model_ident);
    let impl_orderbyinput = impl_orderbyinput(&fields_info, &model_ident);

    Ok(quote! {
        #where_struct

        #impl_whereinput

        #orderby_struct

        #impl_orderbyinput
    })
}
