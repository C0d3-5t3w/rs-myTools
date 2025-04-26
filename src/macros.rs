/// Create a HashMap from key-value pairs
///
/// # Examples
///
/// ```
/// use rs_mytools::map;
///
/// let scores = map! {
///     "Alice" => 42,
///     "Bob" => 36,
///     "Charlie" => 94,
/// };
/// assert_eq!(scores["Bob"], 36);
/// ```
#[macro_export]
macro_rules! map {
    // Empty map
    () => {
        std::collections::HashMap::new()
    };
    
    // Map with entries
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::with_capacity(count!($($key),+));
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

/// Create a HashSet from values
///
/// # Examples
///
/// ```
/// use rs_mytools::set;
///
/// let colors = set!["red", "green", "blue"];
/// assert!(colors.contains("red"));
/// ```
#[macro_export]
macro_rules! set {
    // Empty set
    () => {
        std::collections::HashSet::new()
    };
    
    // Set with entries
    ($($value:expr),+ $(,)?) => {
        {
            let mut set = std::collections::HashSet::with_capacity(count!($($value),+));
            $(
                set.insert($value);
            )+
            set
        }
    };
}

/// Count the number of arguments (used internally)
#[macro_export]
#[doc(hidden)]
macro_rules! count {
    () => (0);
    ($head:expr $(, $tail:expr)*) => (1 + count!($($tail),*));
}

/// Try to execute an expression and return early with the error if it fails
///
/// Similar to the `?` operator but works in functions returning Result<T, String>
/// or other types where the error needs to be converted
///
/// # Examples
///
/// ```
/// use rs_mytools::try_or_return;
///
/// fn process() -> Result<(), String> {
///     let file = try_or_return!(std::fs::File::open("file.txt"), 
///                              |e| format!("Failed to open file: {}", e));
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! try_or_return {
    ($expr:expr, $err_mapper:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err($err_mapper(err)),
        }
    };
}

/// Debug print a value with its name and location
///
/// # Examples
///
/// ```
/// use rs_mytools::dbg_print;
///
/// let x = 42;
/// dbg_print!(x); // prints: [src/main.rs:4] x = 42
/// ```
#[macro_export]
macro_rules! dbg_print {
    ($val:expr) => {
        {
            eprintln!("[{}:{}] {} = {:?}",
                file!(), line!(), stringify!($val), $val);
            $val
        }
    };
    
    ($val:expr, $($arg:tt)+) => {
        {
            eprintln!("[{}:{}] {} = {:?} // {}",
                file!(), line!(), stringify!($val), $val, format!($($arg)+));
            $val
        }
    };
}

/// Create a vec from a sequence of values
///
/// # Examples
///
/// ```
/// use rs_mytools::vec;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(numbers.len(), 5);
/// ```
#[macro_export]
macro_rules! vec {
    // Empty vector
    () => {
        Vec::new()
    };
    
    // Vector with entries
    ($($x:expr),+ $(,)?) => {
        {
            let mut vec = Vec::with_capacity(count!($($x),+));
            $(
                vec.push($x);
            )+
            vec
        }
    };
    
    // Vector with repeated element
    ($elem:expr; $n:expr) => {
        {
            let mut vec = Vec::with_capacity($n);
            vec.resize($n, $elem);
            vec
        }
    };
}

/// A more concise version of match for Options
///
/// # Examples
///
/// ```
/// use rs_mytools::when;
///
/// let opt = Some(42);
/// let result = when!(opt {
///     Some(val) => val * 2,
///     None => 0,
/// });
/// assert_eq!(result, 84);
/// ```
#[macro_export]
macro_rules! when {
    // Pattern with curly braces
    ($value:expr, {
        $($pattern:pat $(if $guard:expr)? => $result:expr),+
        $(, else => $else_result:expr)?
    }) => {
        match $value {
            $($pattern $(if $guard)? => $result,)+
            $(#[allow(unreachable_patterns)] _ => $else_result,)?
        }
    };
    
    // Alternative arrow-style pattern
    ($value:expr => {
        $($pattern:pat $(if $guard:expr)? => $result:expr),+
        $(, else => $else_result:expr)?
    }) => {
        match $value {
            $($pattern $(if $guard)? => $result,)+
            $(#[allow(unreachable_patterns)] _ => $else_result,)?
        }
    };
}

/// Assert multiple conditions at once
///
/// # Examples
///
/// ```
/// use rs_mytools::assert_all;
///
/// let x = 5;
/// let y = 10;
/// assert_all!(
///     x < y,
///     x > 0,
///     y % x == 0
/// );
/// ```
#[macro_export]
macro_rules! assert_all {
    ($($cond:expr),+ $(,)?) => {
        {
            let mut all_passed = true;
            let mut failures = Vec::new();
            
            $(
                if !$cond {
                    all_passed = false;
                    failures.push(stringify!($cond));
                }
            )+
            
            if !all_passed {
                panic!("assertion failed: {:?}", failures);
            }
        }
    };
}
