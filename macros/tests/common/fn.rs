use super::*;

#[test]
fn signature_finds_context_and_stream_in_any_parameter_position() {
    let signatures: [Signature; 3] = [
        parse_quote!(fn handle(stream: &mut Stream, context: &mut Context)),
        parse_quote!(fn handle(self, value: u8, context: &Context, stream: &Stream)),
        parse_quote!(fn handle(context: &mut ::hyperlane::Context, stream: &mut ::hyperlane::Stream)),
    ];
    for signature in signatures {
        assert_eq!(parse_context_from_signature(&signature).unwrap(), "context");
        assert_eq!(parse_stream_from_signature(&signature).unwrap(), "stream");
    }
}

#[test]
fn signature_keeps_the_first_matching_parameter() {
    let signature: Signature =
        parse_quote!(fn handle(first: &Context, second: &Context, stream: &Stream));
    assert_eq!(parse_context_from_signature(&signature).unwrap(), "first");
}

#[test]
fn signature_accepts_wildcard_parameters() {
    let signature: Signature = parse_quote!(fn handle(_: &mut Context, _: &mut Stream));
    assert_eq!(parse_context_from_signature(&signature).unwrap(), "_");
    assert_eq!(parse_stream_from_signature(&signature).unwrap(), "_");
}

#[test]
fn signature_rejects_owned_and_unrelated_types() {
    let signatures: [Signature; 3] = [
        parse_quote!(fn handle(context: Context, stream: Stream)),
        parse_quote!(fn handle(context: &other::Context, stream: &other::Stream)),
        parse_quote!(fn handle(value: usize)),
    ];
    for signature in signatures {
        assert!(parse_context_from_signature(&signature).is_err());
        assert!(parse_stream_from_signature(&signature).is_err());
    }
}

#[test]
fn signature_rejects_destructured_context_and_stream_patterns() {
    let context: Signature = parse_quote!(fn handle((left, right): &Context));
    let stream: Signature = parse_quote!(fn handle((left, right): &Stream));
    assert!(parse_context_from_signature(&context).is_err());
    assert!(parse_stream_from_signature(&stream).is_err());
}

#[test]
fn order_expression_accepts_integer_and_signed_string_literals() {
    let values: [(Expr, &str); 4] = [
        (parse_quote!(0), "Some(0isize)"),
        (parse_quote!(37), "Some(37isize)"),
        (parse_quote!("-7"), "Some(-7isize)"),
        (parse_quote!("19"), "Some(19isize)"),
    ];
    for (value, expected) in values {
        let tokens: TokenStream = expr_to_isize(&Some(value));
        let _: Expr = parse2(tokens.clone()).unwrap();
        assert_eq!(tokens.to_string().replace(' ', ""), expected);
    }
}

#[test]
fn order_expression_ignores_missing_and_unsupported_values() {
    assert_eq!(expr_to_isize(&None).to_string(), "None");
    let values: [Expr; 3] = [parse_quote!(true), parse_quote!(value), parse_quote!(1 + 2)];
    for value in values {
        assert_eq!(expr_to_isize(&Some(value)).to_string(), "None");
    }
}

#[test]
fn context_leak_generation_preserves_the_unsafe_mode() {
    let context: Ident = Ident::new("ctx", Span::call_site());
    assert_eq!(
        leak_context(false, &context).to_string(),
        quote!(unsafe { ctx.leak() }).to_string()
    );
    assert_eq!(
        leak_context(true, &context).to_string(),
        quote!(ctx.leak()).to_string()
    );
    assert_eq!(
        leak_mut_context(false, &context).to_string(),
        quote!(unsafe { ctx.leak_mut() }).to_string()
    );
    assert_eq!(
        leak_mut_context(true, &context).to_string(),
        quote!(ctx.leak_mut()).to_string()
    );
}
