#[macro_export]
macro_rules! option_array {
    // 入口点：匹配数组字面量
    ($($input:tt)*) => {
        $crate::binary_tree::option_array_inner!([] $($input)*)
    };
}
pub use option_array;

#[macro_export]
macro_rules! option_array_inner {
    // 终止条件：所有元素处理完成，返回结果数组
    ([$($output:expr,)*]) => {
        [$($output,)*]
    };

    // 处理 null 元素（后面有逗号）
    ([$($output:expr,)*] null, $($rest:tt)*) => {
        $crate::binary_tree::option_array_inner!([$($output,)* None,] $($rest)*)
    };

    // 处理普通表达式元素（后面有逗号）
    ([$($output:expr,)*] $element:expr, $($rest:tt)*) => {
        $crate::binary_tree::option_array_inner!([$($output,)* Some($element),] $($rest)*)
    };

    // 处理最后一个 null 元素
    ([$($output:expr,)*] null) => {
        $crate::binary_tree::option_array_inner!([$($output,)* None,])
    };

    // 处理最后一个普通表达式元素
    ([$($output:expr,)*] $element:expr) => {
        $crate::binary_tree::option_array_inner!([$($output,)* Some($element),])
    };
}
pub use option_array_inner;

#[macro_export]
macro_rules! btree {
    ($($input:tt)*) => {
        $crate::binary_tree::new_tree_from(&$crate::binary_tree::option_array!($($input)*))
    };
}
pub use btree;
