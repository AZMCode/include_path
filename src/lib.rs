#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::TokenStream;

fn parse_punct(t: TokenStream) -> Option<Vec<String>> {
    let mut output = vec![];
    let mut expect_punct = false;
    for curr_tok in t.into_iter() {
        match curr_tok {
            proc_macro::TokenTree::Literal(val) => {
                if !expect_punct {
                    output.push(val.to_string());
                }
            },
            proc_macro::TokenTree::Punct(val) => {
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

macro_rules! include_path_macro {
    ($original_ident: literal, $new_ident: ident, $for_windows: expr) => {
        #[proc_macro]
        pub fn $new_ident(t: TokenStream) -> TokenStream {
            let include_macro_ident = proc_macro::Ident::new($original_ident, proc_macro::Span::call_site());
            let literals = match parse_punct(t) {
                Some(val) => val,
                None => { return TokenStream::new() }
            };
            if $for_windows {
                TokenStream::from(
                    proc_macro::TokenTree::from(
                        proc_macro::Literal::string(&   literals.join("\\"))
                    )
                )
            } else {
                TokenStream::from(
                    proc_macro::TokenTree::from(
                        proc_macro::Literal::string(&literals.join("/"))
                    )
                )
            }
        }
    }
}
#[cfg( host_family ="windows")]
static for_windows: bool = true;
#[cfg(not(host_family = "windows"))]
static for_windows: bool = false;

include_path_macro!("include",include_path,for_windows);
include_path_macro!("include_bytes",include_path_bytes,for_windows);
include_path_macro!("include_str",include_path_str,for_windows);
