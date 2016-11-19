// macros scpoing and visibility demo:
macro_rules! m1 { () => (()) }
// visible here: m1
mod foo {
    // visible here: m1
    #[macro_export]
    macro_rules! m2 { () => (()) }
    // visible here: m1, m2
}
// visible here: m1
macro_rules! m3 { () => (()) }
// visible here: m1, m3
#[macro_use]
mod bar {
    // visible here: m1, m3
    macro_rules! m4 { () => (()) }
    // visible here: m1, m3, m4
}
// visible here: m1, m3, m4
// when this lib is loaded with macro_use extern crate,
// only m2 will be imported

fn main() {
    // This is roughly how vec! macro looks like
    // macro is a set of pattern-matching arms
    // grammar: https://doc.rust-lang.org/stable/reference.html#macros
    macro_rules! vect {                         // macro name definition
        ( $( $x:expr ),* ) => {                 // x:expr matches any rust def, (),* - 0 or more expressions separated by ,
            {                                   // x is metavariable. expr is fragment specifier
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);          // do it for each matched x
                )*
                temp_vec
            }                                   // {} part of syntax, since there are several statements inside
        };                                      // outer braces can be []., ()
                                                // ; optional after last case
    }

    // Rust tokens appearing in matcher must match exactly.
    macro_rules! foo {
        (x => $e:expr) => (println!("mode x: {}", $e));
        (y => $e:expr) => (println!("mode y: {}", $e));
    }
    foo!(y => 3); // prints "mode y: 3"
    //foo!(z => 3); // error, no rules expected the token 'z'

    // nested repetitions
    // count qualifiers: *: [0..many], +: [1..many]
    // ',' in expressin is a separator, may be anything. Optonal.
    macro_rules! oO {
        (
            $(
                $x:expr; [ $( $y:expr ),* ]
            );*
        ) => {
            &[ $($( $x + $y ),*),* ]
        }
    }
    let a: &[i32] = oO!(10; [1, 2, 3]; 20; [4, 5, 6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    // you can do recursive macros even!
    macro_rules! write_html {
        ($w:expr, ) => (()); // stop condition
        ($w:expr, $e:tt) => (write!($w, "{}", $e)); // single token
        ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
            write!($w, "<{}>", stringify!($tag));
            write_html!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            write_html!($w, $($rest)*);
        }};
    }

    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h6["Macros woohoo"]]
        ]
    );
    println!("{}", out);

    // debuging macros:
    // 'rustc --pretty expanded' - whole crate with expanded macros
    // '--pretty expanded, hygiene' - mult vars with the same name, diff contexts
    // syntax extensions (unstable, reuires feature gates):
    // log_syntax!(...) - print args to stdout # compile time, expand to nothing
    // trace_macros!(true) - compiler message every time macro is expanded. false will turn off.

    // Rust code can be parsed into syntax tree with macros not expanded.
    // Consequences: when parding a macro need to know if it spands for
    // zero+ items, methods, an expression, statement or a pattern
    // * macro that stands for items must be either:
    //   * delimited by {}, like foo! { ... }
    //   * terminated by ;, like foo!(...);
    // * braces inside macro must be balanced

    // Fragment specifiers:
    // * ident - identifier. like 'x;' 'foo'
    // * path - qualified name, like 'T::SpecialA'
    // * expr - an expression. Anything.
    // * ty - a type. Like 'i32', '&T', 'Vec<char>'
    // * pat - pattern, 'Some(t)', '(17, 'a')', '_'
    // * stmt - simgle statement, 'let x = 3'
    // * block - brace-delimited sequence of statements
    // * item - component of crate. 'fn foo() {}', 'struct Bar;'
    // * meta - meta item, attribute, 'cfg(target_os="windows")'
    // * tt - single token tree

    // expr, stmt; ty, path; pat have special requireents regarding following tokens
    // dictated by language syntax
}
