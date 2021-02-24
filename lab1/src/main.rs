use concise_scanf_like_input::{read_input_lines, read_line_of_vars};
use std::io::{self, Stdin, Read};
use std::cmp;

use clap::{self, Arg, App, SubCommand};

/// Greatest Commond Denominator.
/// How to calculate it for two integers?
/// First, let's say we have two integers and the greater of them is to
/// the left from the smaller one: \((a, b)\), \(a > b\)
///
/// Clearly, \(gcd(a, b)\) lies somewhere in this range \([1, b]\).
/// Otherwise, it would not divide \(b\).
///
/// Let's check if \(b\) is the \(gcd(a, b). For that, divide \(a\) by
/// \(b\). If \(b\) divides \(a\), then that is the case by definition.
///
/// If not, then what do we have? From the one hand, we know that the new
/// range of search must be \([1, b - 1]\). From the other hand, we can
/// sift through the possible values by noting the fact that we now know
/// the residue \(a % b\) and that \(gcd(a, b)\) must also divide
/// \(a % b\), as the following scheme illustrates:
///
/// ```
/// |    a % b
/// ---
/// |
/// |    b
/// ---
///      ...
/// ---
/// |
/// |    b
/// ```
///
/// Division of \(a\) by \(b\) with a residue \(a % b\). We are looking
/// for a number \(gcd(a, b)\) -- the maximum integer that fits inside
/// both \(b\) and \(a % b\). Thus, we have reduced the problem to a
/// smaller one, as \(a > b\) and \(a % b < b\). Sooner or later, we
/// finish this process by reaching a pair of numbers such that the
/// smaller divides the greater one. Thus, the algorithm is proven to
/// work. Thus, we always started our search process from the maximum
/// possible value of gcd(a, b) and reached the answer. Thus, we have
/// found it.

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (std::cmp::max(a, b), std::cmp::min(a, b));
    while a % b != 0 {
	let b_t = b;
	b = a % b;
	a = b_t;
    }
    b
}

fn main() {
    let mut buf = String::new();
    let mut lines: &[&str] = read_input_lines!(buf);

    let matches = App::new("Laboratory work #1")
	.author("Artyom Gevorgyan")
	.about("Mathematical foundations of Euclidean's algorithm")
	.arg(
	    Arg::with_name("mode")
		.long("mode")
		.takes_value(true)
		.required(true)
		.possible_values(&["gcd", "extended_gcd", "numerical_experiment"])
	).get_matches();

    match matches.value_of("mode") {
	Some("gcd") => {
	    let (a, b): (i32, i32);
	    read_line_of_vars!(i32, lines, a, b);
	    println!("{}", gcd(a, b));
	},
	Some("extended_gcd") => {
	    println!("extended_gcd");
	},
	Some("numerical_experiment") => {
	    println!("numerical_experiment");
	},
	_ => {
	    println!("sdfsdfs");
	}
    }

    /*
     * 1) d = gcd(a, b) ? simple gcd
     * 2) au + bv = d ? what are u and v? extended gcd
     * 3) as + bt = c ? if c is given. not gcd(a, b), but given.
     */

    let (a, b): (i32, i32);
    // read_line_of_vars!(i32, lines, a, b);
}
