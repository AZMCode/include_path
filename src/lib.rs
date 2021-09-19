//! A cross-platform way to `include!` source, strings and bytes
//! 
//! This crate provides a proposed set of macros to complement the existing
//! `include_*`macros in Rust, taking a variadic set of arguments, combinind them
//! into a platform-specific path string at compilation time, and returning
//! the corresponding underlying macros
//! 
//! # Examples
//! 
//!  ```
//! // This code assumes the file "../res-tests/include.txt" exists and contains the following:
//! // ```
//! // include = "Test String"
//! // ```
//! // This code will compile in both Windows systems and Unix-based systems
//! 
//! use include_path::include_path;
//! let include;
//! include_path!("..","res-tests","include.txt");
//! assert_eq!("Test String",include);
//! ```
//! 
//! ```
//! // This code assumes the file "../res-tests/include_bytes.txt" exists and contains the following UTF-8 encoded text:
//! // ```
//! // Test Bytes
//! // ```
//! // This code will compile in both Windows systems and Unix-based systems
//! 
//! use include_path::include_path_bytes;
//! 
//! let include_bytes = include_path_bytes!("..","res-tests","include_bytes.txt");
//! assert_eq!("Test Bytes".as_bytes(),include_bytes);
//! ```
//! 
//! ```
//! // This code assumes the file "../res-tests/include_str.txt" exists and contains the following:
//! // ```
//! // Test String
//! // ```
//! // This code will compile in both Windows systems and Unix-based systems
//! 
//! use include_path::include_path_str;
//! 
//! let include_str = include_path_str!("..","res-tests","include_str.txt");
//! assert_eq!("Test String",include_str);
//! ```

#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]
extern crate proc_macro;
use proc_macro::*;
use litrs;

// To parse a comma-separated stream of literals
fn parse_punct(t: TokenStream) -> Option<Vec<String>> {
    let mut output = vec![];
    let mut expect_punct = false;
    for curr_tok in t.into_iter() {
        match curr_tok {
            TokenTree::Literal(ref val) => {
                // Here the use of an external crate is needed due to the lack of an interface in
                // proc_macro to check the type and internal value of a Literal token.
                // 
                // `litrs` was chosen over `syn` due to its more lightweight nature
                let parsed_literal = litrs::Literal::from(val);
                match parsed_literal {
                    litrs::Literal::String(parsed_val) => {
                        if expect_punct {
                            return None;
                        }
                        output.push(parsed_val.value().to_string());
                    },
                    _ => {
                        val.span().error("Unexpected punctuation or punctuation type").emit();
                        return None;
                    }
                };
            },
            TokenTree::Punct(val) => {
                if !expect_punct || val.as_char() != ',' {
                    val.span().error("Unexpected punctuation or punctuation type").emit();
                    return None;
                }
            },
            unexpected_token => {
                unexpected_token.span().error("Unexpected token").emit();
                return None;
            }
        }
        expect_punct = !expect_punct;
    }
    Some(output)
}

//Macro to generate the proc_macro code without repetition
macro_rules! include_path_macro {
    ($original_ident: literal, $new_ident: ident, $for_windows: expr) => {
        #[proc_macro]
        pub fn $new_ident(t: TokenStream) -> TokenStream {
            let mut output = TokenStream::from(
                TokenTree::from(
                    Ident::new($original_ident, Span::call_site())
                )
            );
            output.extend_one(
                TokenTree::from(
                    Punct::new('!',Spacing::Alone)
                )
            );
            let literals = match parse_punct(t) {
                Some(val) => val,
                None => { return TokenStream::new() }
            };
            let joined_path;
            if $for_windows {
                joined_path = Literal::string(&   literals.join("\\"));
            } else {
                joined_path = Literal::string(&literals.join("/"));
            }
            output.extend_one(
                TokenTree::from(
                    Group::new(
                        Delimiter::Parenthesis,
                        TokenStream::from(
                            TokenTree::from(joined_path)
                        )
                    )
                )
            );
            output
        }
    }
}

// This is where the desicion of path type is taken. Problems could arise if this crate is compiled
// in one system but used in another.
#[cfg( host_family ="windows")]
static FOR_WINDOWS: bool = true;
#[cfg(not(host_family = "windows"))]
static FOR_WINDOWS: bool = false;

include_path_macro!("include",include_path,FOR_WINDOWS);
include_path_macro!("include_bytes",include_path_bytes,FOR_WINDOWS);
include_path_macro!("include_str",include_path_str,FOR_WINDOWS);