#[macro_export]
macro_rules! errorln(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

// Note:
// string::String::insert_str() is not available because it's unstable.
//   https://github.com/rust-lang/rust/issues/35553
pub fn insert(target: &mut String, index: usize, inserted: &str) {
    for c in inserted.chars().rev() {
        target.insert(index, c);
    }
}

pub type ErrorMsg = String;
pub type Result<T> = ::std::result::Result<T, ErrorMsg>;
