use super::*;

/// Creates a protocol check function for protocol upgrade type validation.
///
/// # Arguments
///
/// - `&proc_macro2::Ident` - The protocol upgrade type identifier as an ident.
///
/// # Returns
///
/// Returns a closure that generates the protocol check code.
pub(crate) fn create_protocol_check(
    upgrade_type: &proc_macro2::Ident,
) -> impl FnOnce(&Ident, &Ident) -> proc_macro2::TokenStream {
    let upgrade_type_str: String = upgrade_type.to_string();
    move |context: &Ident, _: &Ident| {
        let check_fn: proc_macro2::Ident =
            Ident::new(&format!("is_{upgrade_type_str}"), context.span());
        quote! {
            if !#context.get_request().get_upgrade_type().#check_fn() {
                return ::hyperlane::Status::Continue;
            }
        }
    }
}

/// Implements a protocol upgrade type check macro.
///
/// This macro generates a handler function for a specific protocol upgrade type check.
/// It expands to a check that aborts the request if the upgrade type does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$submit_name` - The string name for macro registration as an ident.
/// - `$upgrade_type` - The protocol upgrade type identifier as an ident.
macro_rules! impl_protocol_check_macro {
    ($name:ident, $submit_name:ident, $upgrade_type:ident) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_protocol_check(&proc_macro2::Ident::new(
                    stringify!($upgrade_type),
                    Span::call_site(),
                )),
            )
        }
    };
}

impl_protocol_check_macro!(is_ws_upgrade_type_macro, is_ws_upgrade_type, ws);
impl_protocol_check_macro!(is_h2c_upgrade_type_macro, is_h2c_upgrade_type, h2c);
impl_protocol_check_macro!(is_tls_upgrade_type_macro, is_tls_upgrade_type, tls);
impl_protocol_check_macro!(
    is_unknown_upgrade_type_macro,
    is_unknown_upgrade_type,
    unknown
);
