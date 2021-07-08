use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Type;

mod utils;

#[proc_macro_derive(ToMaps, attributes(label))]
pub fn serialize_to_maps(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#struct_name`.
    let struct_ident = input.ident.clone();
    let struct_gen = input.generics.clone();

    let mut ret = quote! {};

    // TODO: 这里按道理只需要遍历一次 input.data。 但现在会遍历 map 数加一次。 不过这是编译期的步骤，不影响运行时性能，问题不大。

    // 先遍历找出调用者打了多少个 Labels 。对每个 Label 都生成一个相应的 to_xxx 方法。 调用既可得到HashMap。
    maps(&input.data).into_iter().for_each(|map_name| {
        let insert_maps = insert_maps(&input.data, &map_name);
        let map_ident = syn::Ident::new(&format!("to_{}", map_name), struct_ident.span());
        ret.extend(quote! {
            impl#struct_gen #struct_ident#struct_gen {
                pub fn #map_ident(&self) -> std::collections::HashMap<String,String> {
                    let mut map = std::collections::HashMap::new();
                    #insert_maps
                    map
                }
            }
        });
    });
    ret.into()
}

pub(crate) fn maps(data: &Data) -> HashSet<String> {
    if let Data::Struct(ref data) = data {
        if let Fields::Named(ref fields) = data.fields {
            fields
                .named
                .iter()
                .fold(HashSet::new(), |mut map_names, f| {
                    if f.ident.is_some() {
                        if let Some(map_name) = utils::the_first_attr(f, "label") {
                            map_names.insert(map_name);
                        }
                    }
                    map_names
                })
        } else {
            panic!("Field should have a name.")
        }
    } else {
        panic!("This macro could not be used on types other than struct.")
    }
}

pub(crate) fn insert_maps(data: &Data, map_name: &str) -> quote::__private::TokenStream {
    let mut ret = quote! {};
    if let Data::Struct(ref data) = data {
        if let Fields::Named(ref fields) = data.fields {
            fields.named.iter().for_each(|f| {
                if let Some(name) = &f.ident {
                    if utils::the_first_attr(f, "label")
                        .and_then(|attr_val| {
                            if &attr_val == map_name {
                                Some(())
                            } else {
                                None
                            }
                        })
                        .is_some()
                    {
                        if is_option(&f) {
                            ret.extend( quote!{
                                if let Some(_val) = &self.#name {
                                    map.insert(stringify!(#name).to_ascii_lowercase().to_string().replace("_","-"), _val.to_string());
                                }
                            })
                        } else{
                        ret.extend(quote! {
                            map.insert(stringify!(#name).to_ascii_lowercase().to_string().replace("_","-"), self.#name.to_string());
                        })}
                    }
                }
            });
        } else {
            panic!("Field should have a name.")
        }
    } else {
        panic!("This macro could not be used on types other than struct.")
    }
    ret
}
/// Check if the filed is Option<T> type
fn is_option(f: &syn::Field) -> bool {
    let typ = &f.ty;

    let opt = match typ {
        Type::Path(typepath) if typepath.qself.is_none() => Some(typepath.path.clone()),
        _ => None,
    };

    if let Some(_path) = opt {
        let idents_of_path = _path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push(':');
            acc
        });
        vec!["Option:", "std:option:Option:", "core:option:Option:"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| _path.segments.last())
            .is_some()
    } else {
        false
    }
}