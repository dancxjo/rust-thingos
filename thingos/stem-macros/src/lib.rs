extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = &func.sig.ident;

    let accepts_arg = match validate_signature(&func) {
        Ok(accepts_arg) => accepts_arg,
        Err(err) => return err.to_compile_error().into(),
    };

    let call_user = if accepts_arg {
        quote! { #fn_name(__stem_arg) }
    } else {
        quote! { #fn_name() }
    };

    let expanded = quote! {
        #func

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn stem_user_main(__stem_arg: usize) -> ! {
            #call_user
        }
    };

    expanded.into()
}

fn validate_signature(func: &ItemFn) -> Result<bool, syn::Error> {
    if func.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[stem::main] does not support async functions",
        ));
    }

    let is_never = matches!(
        func.sig.output,
        ReturnType::Type(_, ref ty) if matches!(**ty, Type::Never(_))
    );
    if !is_never {
        return Err(syn::Error::new_spanned(
            &func.sig.output,
            "#[stem::main] functions must return !",
        ));
    }

    match func.sig.inputs.len() {
        0 => Ok(false),
        1 => {
            let arg = &func.sig.inputs[0];
            if let FnArg::Typed(pat_ty) = arg {
                match pat_ty.ty.as_ref() {
                    Type::Path(path) if path.path.is_ident("usize") => Ok(true),
                    _ => Err(syn::Error::new_spanned(
                        &func.sig.inputs,
                        "expected argument type usize",
                    )),
                }
            } else {
                Err(syn::Error::new_spanned(
                    &func.sig.inputs,
                    "unsupported argument pattern",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            &func.sig.inputs,
            "expected zero or one argument",
        )),
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::validate_signature;

    fn parse_fn(src: &str) -> syn::ItemFn {
        parse_str(src).expect("valid function")
    }

    #[test]
    fn accepts_no_arg_main() {
        let f = parse_fn("fn app() -> ! { loop {} }");
        assert_eq!(validate_signature(&f).expect("valid"), false);
    }

    #[test]
    fn accepts_single_usize_arg_main() {
        let f = parse_fn("fn app(arg: usize) -> ! { let _ = arg; loop {} }");
        assert_eq!(validate_signature(&f).expect("valid"), true);
    }

    #[test]
    fn rejects_async_function() {
        let f = parse_fn("async fn app() -> ! { loop {} }");
        let err = validate_signature(&f).expect_err("must reject async");
        assert!(err.to_string().contains("does not support async"));
    }

    #[test]
    fn rejects_non_never_return_type() {
        let f = parse_fn("fn app() -> i32 { 0 }");
        let err = validate_signature(&f).expect_err("must reject non-never return");
        assert!(err.to_string().contains("must return !"));
    }

    #[test]
    fn rejects_non_usize_argument() {
        let f = parse_fn("fn app(arg: u32) -> ! { let _ = arg; loop {} }");
        let err = validate_signature(&f).expect_err("must reject non-usize arg");
        assert!(err.to_string().contains("expected argument type usize"));
    }

    #[test]
    fn rejects_multiple_arguments() {
        let f = parse_fn("fn app(a: usize, b: usize) -> ! { let _ = (a, b); loop {} }");
        let err = validate_signature(&f).expect_err("must reject multiple args");
        assert!(err.to_string().contains("expected zero or one argument"));
    }
}
