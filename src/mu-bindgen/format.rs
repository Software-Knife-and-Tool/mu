use {
    crate::syntax::Syntax,
    syn::{self, punctuated::Pair, token::Comma, FnArg, Pat, ReturnType, Signature, Visibility},
};

pub trait Format {
    fn fn_arg_signature(_: &Signature) -> String;
    fn fn_attrs<'a>(_: &'a Signature, _: &'a Visibility) -> [&'a str; 4];
    fn fn_return_signature(_: &Signature) -> String;
    fn fn_return_type(_: &ReturnType) -> Option<String>;
}

impl Format for Syntax {
    fn fn_arg_signature(sig: &Signature) -> String {
        sig.inputs
            .pairs()
            .map(|pair: Pair<&FnArg, &Comma>| {
                let fn_arg = match pair.value() {
                    FnArg::Receiver(_rec) => "self".to_string(),
                    FnArg::Typed(type_) => match &*type_.pat {
                        Pat::Ident(id) => {
                            let ref_ = match id.by_ref {
                                Some(_) => "ref ".to_string(),
                                _ => "".to_string(),
                            };

                            let mut_ = match id.mutability {
                                Some(_) => "mut ".to_string(),
                                _ => "".to_string(),
                            };

                            let type_ = match id.subpat {
                                std::option::Option::None => "()".to_string(),
                                Some((_, ref pat)) => match **pat {
                                    Pat::Ident(ref id) => id.ident.to_string(),
                                    _ => "<type>".to_string(),
                                },
                            };

                            format!("{ref_}{mut_}{}: {type_}", id.ident)
                        }
                        _ => "????".to_string(),
                    },
                };

                match pair.punct() {
                    std::option::Option::None => fn_arg,
                    Some(_) => format!("{fn_arg}, "),
                }
            })
            .collect::<String>()
    }

    fn fn_attrs<'a>(sig: &'a Signature, vis: &'a Visibility) -> [&'a str; 4] {
        [
            if sig.constness.is_some() {
                "const "
            } else {
                ""
            },
            if sig.asyncness.is_some() {
                "async "
            } else {
                ""
            },
            if sig.unsafety.is_some() {
                "unsafe "
            } else {
                ""
            },
            if matches!(vis, Visibility::Public(_)) {
                "pub "
            } else {
                ""
            },
        ]
    }

    fn fn_return_signature(sig: &Signature) -> String {
        match Syntax::fn_return_type(&sig.output.clone()) {
            std::option::Option::None => "".to_string(),
            Some(str) => format!(" -> {}", str),
        }
    }

    fn fn_return_type(arg: &ReturnType) -> Option<String> {
        match arg {
            ReturnType::Default => std::option::Option::None,
            ReturnType::Type(_, type_) => Some(format!("{}", Syntax::Type(*type_.clone()))),
        }
    }
}

#[cfg(test)]
mod tests {}
