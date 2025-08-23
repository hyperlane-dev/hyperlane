use crate::*;

/// Returns the order of a hook if it is applicable for duplicate checking.
///
/// This function extracts the priority (`order`) from hooks that are
/// order-sensitive, including:
/// - `RequestMiddleware`
/// - `ResponseMiddleware`
/// - `PanicHook`
/// - `ConnectedHook`
/// - `PreUpgradeHook`
///
/// Hooks with an order of 0 are ignored, as 0 indicates no specific ordering.
///
/// # Arguments
///
/// - `&HookType` - A reference to the `HookType` to extract the order from.
///
/// # Returns
///
/// - `Option<isize>` - `Some(order)` if the hook has a non-zero order, `None` otherwise.
fn hook_order(hook: &HookType) -> Option<isize> {
    match hook {
        HookType::RequestMiddleware(order)
        | HookType::ResponseMiddleware(order)
        | HookType::PanicHook(order)
        | HookType::ConnectedHook(order)
        | HookType::PreUpgradeHook(order) => {
            if *order != 0 {
                Some(*order)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Checks for duplicate hooks with the same type and non-zero order.
///
/// This function iterates over all provided hooks and ensures that no two
/// hooks of the same type share the same non-zero order. If a duplicate is found,
/// the function will panic at runtime.
///
/// # Arguments
///
/// - `&[&HookMacro]` - A slice of references to `HookMacro` structs to check.
///
/// # Panics
///
/// - Panics if a duplicate hook is detected for a given type and order.
pub fn check_duplicate_hooks(hooks: &[&HookMacro]) {
    let mut seen: HashSet<(Discriminant<HookType>, isize)> = HashSet::new();
    hooks
        .iter()
        .filter_map(|hook| {
            hook_order(&hook.hook_type).map(|order| (discriminant(&hook.hook_type), order))
        })
        .for_each(|key| {
            if !seen.insert(key) {
                panic!("Duplicate hook detected: {:?} with order {}", key.0, key.1);
            }
        });
}
