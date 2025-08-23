use crate::*;

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
        .filter_map(|hook| hook.hook_type.get().map(|order| (hook.hook_type, order)))
        .for_each(|(key, order)| {
            if !seen.insert((key, order)) {
                panic!("Duplicate hook detected: {} with order {}", key, order);
            }
        });
}
