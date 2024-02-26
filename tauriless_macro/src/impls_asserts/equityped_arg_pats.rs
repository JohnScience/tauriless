pub(crate) struct EquitypedArgPats {
    // The values are boxed because they're stored this way in syn::PatType,
    // from which they're extracted.
    pub(crate) ty: Box<syn::Type>,
    /// Argument patterns for the type. E.g. `x` in `fn f(x: i32)`
    /// or `WrapperType(x)` in `fn f(WrapperType(x): WrapperType<i32>)`.
    pub(crate) arg_pats: Vec<Box<syn::Pat>>,
}

impl EquitypedArgPats {
    // The `.new()` is used inside of `HashMap::or_insert`, so making the vec empty is more convenient.
    pub(crate) fn new(ty: Box<syn::Type>) -> Self {
        let arg_pats = vec![];
        Self { ty, arg_pats }
    }

    pub(crate) fn push_arg_pat(&mut self, arg_pat: Box<syn::Pat>) {
        self.arg_pats.push(arg_pat);
    }
}
