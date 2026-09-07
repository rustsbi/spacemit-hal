//! Checked entry attribute for spacemit-rt.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, Type, Visibility, parse_macro_input, spanned::Spanned};

/// Declares the boot-hart entry receiving the selected SoC's peripherals.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if let Err(error) = validate(args.into(), &f) {
        return error.to_compile_error().into();
    }

    let runtime = match proc_macro_crate::crate_name("spacemit-rt") {
        Ok(proc_macro_crate::FoundCrate::Itself) => quote!(::spacemit_rt),
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let name = syn::Ident::new(&name, f.span());
            quote!(::#name)
        }
        Err(error) => return syn::Error::new(f.span(), error).to_compile_error().into(),
    };

    expand(f, runtime).into()
}

fn validate(args: Tokens, f: &ItemFn) -> syn::Result<()> {
    let sig = &f.sig;
    let input = matches!(sig.inputs.first(), Some(FnArg::Typed(arg))
        if matches!(arg.ty.as_ref(), Type::Path(_)));
    let output = match &sig.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => {
            matches!(ty.as_ref(), Type::Never(_))
                || matches!(ty.as_ref(), Type::Tuple(tuple) if tuple.elems.is_empty())
        }
    };
    if !args.is_empty()
        || sig.inputs.len() != 1
        || !input
        || sig.constness.is_some()
        || sig.asyncness.is_some()
        || sig.unsafety.is_some()
        || sig.abi.is_some()
        || !sig.generics.params.is_empty()
        || sig.generics.where_clause.is_some()
        || sig.variadic.is_some()
        || !matches!(f.vis, Visibility::Inherited)
        || !output
    {
        return Err(syn::Error::new(
            f.span(),
            "expected a private function fn(Peripherals) with return type () or !",
        ));
    }
    for attr in &f.attrs {
        if !attr.path().is_ident("doc")
            && !attr.path().is_ident("allow")
            && !attr.path().is_ident("deny")
            && !attr.path().is_ident("warn")
            && !attr.path().is_ident("forbid")
        {
            return Err(syn::Error::new(
                attr.span(),
                "unsupported entry attribute; place cfg before #[entry]",
            ));
        }
    }
    Ok(())
}

fn expand(f: ItemFn, runtime: Tokens) -> Tokens {
    let name = &f.sig.ident;

    quote!(
        #[unsafe(export_name = "__spacemit_rt_main")]
        unsafe extern "C" fn #name() {
            #f

            // SAFETY: Only the boot hart calls this entry once; the startup
            // contract establishes the selected SoC's peripheral ownership and access.
            #name(unsafe { #runtime::Peripherals::steal() });
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_unit_and_diverging_entries() {
        for source in [
            "fn main(p: Peripherals) {}",
            "fn main(mut p: Peripherals) -> () {}",
            "fn main(p: Peripherals) -> ! { loop {} }",
            "fn boot(p: spacemit_rt::Peripherals) {}",
            "fn boot(p: Alias) {}",
        ] {
            let f = syn::parse_str(source).unwrap();
            validate(Tokens::new(), &f).unwrap();
            syn::parse2::<syn::File>(expand(f, quote!(::spacemit_rt))).unwrap();
        }
    }

    #[test]
    fn rejects_invalid_signatures_and_attributes() {
        for source in [
            "pub fn main(p: Peripherals) {}",
            "async fn main(p: Peripherals) {}",
            "const fn main(p: Peripherals) {}",
            "unsafe fn main(p: Peripherals) {}",
            "extern \"C\" fn main(p: Peripherals) {}",
            "fn main() {}",
            "fn main(p: &mut Peripherals) {}",
            "fn main(p: Peripherals, other: Peripherals) {}",
            "fn main<T>(p: Peripherals) {}",
            "fn main(p: Peripherals) -> u32 { 0 }",
            "#[unsafe(no_mangle)] fn main(p: Peripherals) {}",
            "#[cfg(any())] fn main(p: Peripherals) {}",
        ] {
            assert!(
                validate(Tokens::new(), &syn::parse_str(source).unwrap()).is_err(),
                "{source}"
            );
        }
        assert!(
            validate(
                quote!(argument),
                &syn::parse_quote!(
                    fn main(p: Peripherals) {}
                )
            )
            .is_err()
        );
    }

    #[test]
    fn preserves_function_name_with_fixed_export() {
        let f = syn::parse_quote!(
            fn boot(mut p: Peripherals) {}
        );
        let expanded = syn::parse2::<syn::File>(expand(f, quote!(::spacemit_rt))).unwrap();
        assert_eq!(expanded.items.len(), 1);
        let syn::Item::Fn(entry) = &expanded.items[0] else {
            panic!("expected an entry function");
        };
        assert_eq!(entry.sig.ident, "boot");
        assert!(matches!(entry.sig.output, ReturnType::Default));
        assert!(entry.sig.unsafety.is_some());
        assert!(entry.sig.inputs.is_empty());
        let syn::Stmt::Item(syn::Item::Fn(user)) = &entry.block.stmts[0] else {
            panic!("expected the user's function inside the trampoline");
        };
        assert!(user.sig.unsafety.is_none());
        assert_eq!(user.sig.inputs.len(), 1);
        let attribute = &entry.attrs[0];
        assert_eq!(
            quote!(#attribute).to_string(),
            quote!(#[unsafe(export_name = "__spacemit_rt_main")]).to_string()
        );
    }
}
