use crate::*;

/// Extract the optional priority (`order`) from a hook type.
///
/// This function is used to determine the order of hooks that are
/// sensitive to execution priority. The following hook types may carry
/// an `order` value:
/// - `RequestMiddleware`
/// - `ResponseMiddleware`
/// - `PanicHook`
/// - `ConnectedHook`
/// - `PreUpgradeHook`
///
/// Hooks that do not provide an `order` are treated as having no specific
/// priority and will be ignored in duplicate checks.
///
/// # Parameters
/// - `&HookType` - A reference to the `HookType` to extract the order from.
///
/// # Returns
/// - `Option<isize>` - `Some(order)` if the hook defines a priority, otherwise `None`.
fn hook_order(hook: &HookType) -> Option<isize> {
    match hook {
        HookType::RequestMiddleware(order)
        | HookType::ResponseMiddleware(order)
        | HookType::PanicHook(order)
        | HookType::ConnectedHook(order)
        | HookType::PreUpgradeHook(order) => *order,
        _ => None,
    }
}

/// Verify that hooks with the same type and non-zero priority are unique.
///
/// This function iterates over all provided hooks and ensures that no two
/// hooks of the same type define the same non-zero `order`. If a duplicate
/// is found, the function will panic at runtime.
///
/// # Parameters
/// - `&[&HookMacro]` - A slice of references to `HookMacro` structs to be checked.
///
/// # Panics
/// - Panics if two or more hooks of the same type define the same non-zero `order`.
pub fn assert_hooks_unique_order(hooks: &[&HookMacro]) {
    let mut seen: HashSet<(HookType, isize)> = HashSet::new();
    hooks
        .iter()
        .filter_map(|hook| hook_order(&hook.hook_type).map(|order| (hook.hook_type, order)))
        .for_each(|(key, order)| {
            if !seen.insert((key, order)) {
                panic!("Duplicate hook detected: {} with order {}", key, order);
            }
        });
}
