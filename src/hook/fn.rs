use crate::*;

/// Verify that a `Hook` list with the same type and non-zero priority are unique.
///
/// This function iterates over all provided `Hook` items and ensures that no two
/// `Hook` items of the same type define the same non-zero `order`. If a duplicate
/// is found, the function will panic at runtime.
///
/// # Arguments
///
/// - `&[HookMacro]`: A slice of `HookMacro` instances to be checked.
///
/// # Panics
///
/// - Panics if two or more `Hook` items of the same type define the same non-zero `order`.
pub fn assert_hook_unique_order(list: &[HookMacro]) {
    let mut seen: HashSet<(HookType, isize)> = HashSet::new();
    list.iter()
        .filter_map(|hook| hook.hook_type.get().map(|order| (hook.hook_type, order)))
        .for_each(|(key, order)| {
            if !seen.insert((key, order)) {
                panic!("Duplicate hook detected: {} with order {}", key, order);
            }
        });
}
