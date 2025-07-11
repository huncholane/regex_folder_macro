#![doc=include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
#[doc = include_str!("../docs/load_regex_files.md")]
pub fn load_regex_files(input: TokenStream) -> TokenStream {
    let folder_path = parse_macro_input!(input as LitStr).value();

    let folder_path =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(folder_path);

    // Read files from the directory
    let files = fs::read_dir(folder_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("re") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Generate enum variants and compiled regex objects
    let variants = files.iter().map(|path| {
        let base = path.file_stem().unwrap().to_str().unwrap();
        let variant_name = &format!("{}RE", base);
        let ident = Ident::new(variant_name, proc_macro2::Span::call_site());
        quote! {
            pub #ident: regex::Regex
        }
    });

    // Generate struct initialization
    let init_fields = files.iter().map(|path| {
        let base = path.file_stem().unwrap().to_str().unwrap();
        let variant_name = &format!("{}RE", base);
        let ident = Ident::new(variant_name, proc_macro2::Span::call_site());
        let file_path = path.to_str().unwrap();
        quote! {
            #ident: regex::Regex::new(&format!(r"(?mx){}", include_str!(#file_path))).unwrap()
        }
    });

    // Generate the enum and regex accessor
    let expanded = quote! {
        pub struct RegexFile {
            #(#variants),*
        }

        impl RegexFile {
            pub fn new() -> Self {
                RegexFile {
                    #(#init_fields),*
                }
            }
        }

        lazy_static::lazy_static! {
            pub static ref RE: RegexFile = RegexFile::new();
        }


    };

    let structs = files.iter().map(|path| {
        let base = path.file_stem().unwrap().to_str().unwrap();
        let variant_name = &format!("{}RE", base);
        let ident = Ident::new(variant_name, proc_macro2::Span::call_site());
        let file_text = fs::read_to_string(path).unwrap();
        let re_str = format!(r"(?mx){}", file_text);
        let re = regex::Regex::new(&re_str).unwrap();

        // Collect fields into a Vec so they can be used multiple times
        let fields: Vec<_> = re
            .capture_names()
            .flatten()
            .map(|name| {
                let ident = Ident::new(name, proc_macro2::Span::call_site());
                quote! {
                    pub #ident: Option<SimpleMatch>
                }
            })
            .collect();

        let field_initializers: Vec<_> = re
            .capture_names()
            .flatten()
            .map(|name| {
                let ident = Ident::new(name, proc_macro2::Span::call_site());
                quote! {
                    #ident: SimpleMatch::from_match(captures.name(#name))
                }
            })
            .collect();

        quote! {
            #[derive(Debug, Clone, serde::Serialize)]
            pub struct #ident {
                pub start_pos: usize,
                pub end_pos: usize,
                pub val: String,
                #(#fields),*
            }

            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
                }
            }

            impl #ident {
                /// Gets the raw captures
                pub fn captures(text:&str) -> Option<regex::Captures<'_>> {
                    let re = &RE.#ident;
                    re.captures(text)
                }

                /// Gets the raw captures from the contents of a file
                /// Buffer is required because captures use references to the original text.
                pub fn captures_from_file<'a>(buf: &'a mut String, filename: &str) -> Result<Option<regex::Captures<'a>>, std::io::Error> {
                    buf.clear();
                    buf.push_str(&std::fs::read_to_string(filename)?);
                    Ok(Self::captures(buf))
                }

                /// Gets the raw captures iter
                pub fn captures_iter(text: &str) -> impl Iterator<Item = regex::Captures<'_>> + '_ {
                    let re = &RE.#ident;
                    re.captures_iter(text)
                }

                /// Gets the raw captures iter from the contents of a file
                /// Buffer is required because captures use references to the original text.
                pub fn captures_iter_from_file<'a>(buf: &'a mut String, filename: &str) -> Result<impl Iterator<Item = regex::Captures<'a>> + 'a, std::io::Error> {
                    buf.clear();
                    buf.push_str(&std::fs::read_to_string(filename)?);
                    Ok(Self::captures_iter(buf))
                }

                /// Extracts the first regex match for the given string
                /// This match contains `start_pos`, `end_pos`, and `val`
                pub fn from_str(text: &str) -> Option<Self> {
                    if let Some(captures) = Self::captures(text) {
                        Some(Self {
                            start_pos: captures.get(0).unwrap().start(),
                            end_pos: captures.get(0).unwrap().end(),
                            val: captures.get(0).unwrap().as_str().to_string(),
                            #(#field_initializers),*
                        })
                    } else {
                        None
                    }
                }

                /// Extracts the first regex match for the text content of the given filename
                /// This match contains `start_pos`, `end_pos`, and `val`
                pub fn from_file(filename: &str) -> Result<Option<Self>, std::io::Error> {
                    let text = std::fs::read_to_string(filename)?;
                    Ok(Self::from_str(&text))
                }

                /// Extracts all regex matches for the given string
                /// Each match contains a `start_pos`, `end_pos`, and each field for the given class contains `start_pos`, `end_pos`, and `val`
                pub fn iter_from_str(text: &str) -> impl Iterator<Item = Self> + '_ {
                    Self::captures_iter(text)
                        .map(|captures| {
                            Self {
                                start_pos: captures.get(0).unwrap().start(),
                                end_pos: captures.get(0).unwrap().end(),
                                val: captures.get(0).unwrap().as_str().to_string(),
                                #(#field_initializers),*
                            }
                        })
                }

                /// Extracts all regex matches for the given string
                /// Each match contains a `start_pos`, `end_pos`, and each field for the given class contains `start_pos`, `end_pos`, and `val`
                pub fn vec_from_str(text: &str) -> Vec<Self> {
                    Self::iter_from_str(text).collect()
                }

                /// Extracts all regex matches for the text contents of a given file
                /// Each match contains a `start_pos`, `end_pos`, and each field for the given class contains `start_pos`, `end_pos`, and `val`
                /// Buffer is required because the iterator refers to the original text.
                pub fn iter_from_file<'a>(buf: &'a mut String, filename: &str) -> Result<impl Iterator<Item = Self> + 'a, std::io::Error> {
                    buf.clear();
                    buf.push_str(&std::fs::read_to_string(filename)?);
                    Ok(Self::iter_from_str(buf))
                }

                /// Extracts all regex matches for the text contents of a given file
                /// Each match contains a `start_pos`, `end_pos`, and each field for the given class contains `start_pos`, `end_pos`, and `val`
                pub fn vec_from_file(filename: &str) -> Result<Vec<Self>, std::io::Error> {
                    let text = std::fs::read_to_string(filename)?;
                    Ok(Self::vec_from_str(&text))
                }
            }
        }
    });

    let expanded = quote! {
        #expanded
        #(#structs)*

        #[derive(Debug, Clone, serde::Serialize)]
        pub struct SimpleMatch {
            pub start_pos: usize,
            pub end_pos: usize,
            pub val: String,
        }

        impl SimpleMatch {
            pub fn from_match(m: Option<regex::Match<'_>>) -> Option<Self> {
                if let Some(m) = m {
                    Some(Self {
                        start_pos: m.start(),
                        end_pos: m.end(),
                        val: m.as_str().to_string(),
                    })
                } else {
                    None
                }
            }

            pub fn to<T: std::str::FromStr>(&self) -> Option<T> {
                self.val.parse::<T>().ok()
            }
        }
    };

    TokenStream::from(expanded)
}
