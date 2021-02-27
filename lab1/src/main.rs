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

/// Extended GCD.
/// Let's remind ourselves of what the general GCD looks like:
/// say we are asked to find gcd(12, 8)
/// we know 8 fits inside 12 1 time.
/// we know the rest is 4
/// say we try to check if gcd(12, 8) is 4 (it could be 1 indeed), and we make sure it is the case
/// then we can use the calculation to find how many times gcd fits inside 12 or 8, not performing
/// any division. Say we want to know about 12. We see that 12 is 8 + 4. We know the gcd is 4, we take
/// 12 and say that we must fit 4 2 * 1 + 1 times in it. This is it. But we can never figure out how to
/// represent 4 in terms of 12 and 8 over the field of non-negative integers.

/// Let's say that 4 fits in 8 2 times. Okay, so we take that.
/// Then, 4 fits in 12, how many times? 12 = 8 + 4, so 3 times.
/// So now we have 3 * 4 = 12, and we need to balance that off with the 8, so that we target the gcd.
/// We are lucky that the gcd is 4, and not 3, 2, or 1. We would have to continue this process otherwise.
/// So here is the requested representation of 4:
/// 12 - 8. Easy.

/// What happens when there are 3 steps in the process? This really looks like inclusion-exclusion principle.
/// Let's artificially add more steps to our example with 12 and 8 by making one of the numbers a prime.
/// Now we take 12 and 17:
/// 17 = 12 * 1 + 5
/// 12 = 5  * 2 + 2
/// 5  = 2  * 2 + 1
/// 2  = 2  * 1 + 0
///
/// 17, 5
/// 17 = 5 * 3 + 2
/// We must fit 2 inside 17, and insde 5 now. We note that we will have to split 2 into 2 ones to do that.
/// We must note that

/// All the residues can be themselves be divided by the gcd(a,b)
/// 12 * 1 + 5 must all be divided by the gcd. We can note that if 5 is a prime then we don't have to
/// continue.

/// The best way to understand how to solve the problem is to visualize it.
/// Euclidean's algorithms is a cool thing, just like the Pascal's triangle.
///
/// Let's say we are given (17, 5)
/// {   3   }
/// 5 | 5 | 5           | 2     (step 1: calculate how many 5's are in 17)
///         {    2    }
///         2    |    2     | 1 (step 2: calculate how many 2's are in 5)
///             {    2    }
///             1    |    1     (step 3: calculate how many 1's are in 2)
///
/// Now we know for sure that 1 is the maximal number that fits inside 17 in a row, and inside 5 in a row.
/// Nothing greater than it will do.
/// What else do we know? From the picture, how can we represent 1, the gcd(17, 5)?
///  2  |  2  | 1
/// 1 1 | 1 1
///             1 =  2 |  2 | 1
///               - 1 1| 1 1|
///               =           1
/// .,... this is really the remnant, not the gcd in terms of the other thigns. That is why it is important
/// to note the objects visually and count them, not just perform arithmetics. the results can match, but
/// still the calculation can be wrong.
/// Thus, we have just unrolled the last pass of the Euclidean's algorithm to represent 1, the gcd(5, 2),
/// in terms of the data we have: 5 - 2 * 2. Done
/// Then let's take ourselves one step further and fit this equation that represents 1, inside the
/// following part of the picture (17, 5):
/// 5 | 5 | 5 | 2
/// I | I | I |
///
/// Say we have:
/// (17, 5)
/// 17 = 5 * 3 + 2
/// 5  = 2 * 2 + 1
/// We are looking for a form of equation that has 5 and 17 in it. And also 1, the gcd(17, 5).
/// 17 = 5 * 3 + 2
/// 5  = 2 * 2 + 1
/// 2  = 2 * 1 + 0 DONE
///
/// Trying to fit 5, 2, 1 inside the numbers. If we omit a possible 5, 2 or 1, then the algorithms is
/// corrupted. So we have to be careful. This is what the Euclidean's algorithm does. We also try to
/// Let's unroll the computation now.
/// 1) 1  = 5 - 2 * 2
///    not that 17 = 5 * 3 + 2
///    so if we prepare 17 = 5 * 3 + 2 for the addition by aligning the 2's in it, we can get rid of the 2's
///    by cancelling them out.
///    2 * 17 = 5 * 6 + 2 * 2
///    then we can add the 1) to it and get:
///    1 + 2 * 17 = 7 * 6,
/// DONE
/// TODO Visualize with the picture above, show how you shrink things to fit them
/// Let's work with a and b, and suppose that the process takes 2 steps, that is, we find the gcd(a,b) in
/// 2 steps
/// a = b  * k1 + m1 | step 1
/// b = m1 * k2 + m2 | step 2 (m2 = 0 at this point, so m1 is the gcd(a, b))
/// Reverse steps (preserve m1):
/// a - b * k1  = m1 | easy
///
/// Now let's try with 2 steps indeed:
/// a  = b  * k1 + m1 | step 1
/// b  = m1 * k2 + m2 | step 2
/// m1 = m2 * k3 + m3 | step 3 (m3 = 0 at this point, so m2 is the gcd(a,b))
/// Reverse steps (preserve m2):
/// 1) m2 = b  - m1 * k2 | step 1 (turn around m2, we will be pushing it upwards)
/// 2) the task is to cancel m1 out, so here is what we do
/// a  = b * k1 + m1
/// m2 = b - m1 * k2
/// - remove m1
/// k2 * (a) = k2 * (b * k1 + m1) // mulitply by k2 both sides of the upper equation
/// use the linear algebra package from 3blue1brown to examplify this whole algorithm and send him a video.
/// Now add the lower equation to the upper one
/// k2 * (a) - m2 = k2 * (b * k1 + m1) + b - m1 * k2
///               = k2 * b * k1 + k2 * m1 + b - m1 * k2 =
///               = k2 * b * k1 + b = (k2 + k1) * b
/// =>
/// k2 * a - m2 = (k2 + k1) * b // Good
/// We were using this: m2 = b - m1 * k2
///
/// Now let's try 4 (5 with the last one) steps and devise a formula
/// a  = b *  k1 + m1
/// b  = m1 * k2 + m2
/// m1 = m2 * k3 + m3
/// m2 = m3 * k4 + m4
/// m3 = m4 * k5 + m5 | m5 = 0, so m4 is the gcd(a,b)
/// Need to do this as a stream
/// Now store the matrix in a table.
/// Start the reverse process.
/// First, let's reverse the first-before-the-last equation to represent the gcd(a,b):
/// m4 = m2 - m3 * k4
/// Now, propagate this equation upwards one time. We have m2 and m3, and that is it. All we have is m2 and m3.
/// The upper equation relates to our equation because it has m2 and m3 too. So we can adjust the coefficients'
/// between them so that we have one of them canceled. Easy.
/// We can remove m3, so that we can link with m2 with the upper-upper equation.
/// So we remove m3. We have k4 * m3 in the lower one. We have just a single m3 in the upper one.
/// So take the upper equation and multiply it by k4:
/// k4 * (m1) = k4 * (m2 * k3 + m3) = k4 * m2 * k3 + k4 * m3
/// m4 = m2 - m3 * k4
/// Add them up:
/// k4 * m1 + m4 = k4 * m2 * k3 + m2 = (k4 + 1 + k3) * m2
/// So we have m4 in here, m2, m1, and m4. Represent it all in terms of these three values and do the same
/// unti you get a, b, m4.
fn extended_gcd(a: i32, b: i32) -> (i32, i32) {
    // Perform the complete gcd process and store the intermediate subproblems in a table
    let (mut a, mut b) = (std::cmp::max(a, b), std::cmp::min(a, b));
    println!("{} {}", a, b);
    let mut vec: Vec<(i32, i32, i32, i32)> = vec![];
    loop {
	if b == 0 {break;}
	let div_coef = a / b;
	let div_rem  = a - div_coef * b;
	vec.push((a, b, div_coef, div_rem));
	a = b;
	b = div_rem;
    }
    println!("logs from extended_gcd {:?}", vec);
    // Now start the reverse process. The last row in the table contains the equation
    // that has zero remainder. Let's get rid of the last equation as it does not
    // contain any remainer, it proved that we have found gcd and has no other
    // informaiton:
    vec.pop();
    // Eventually, we arrive at a place where we have only three values, and the first
    // and the second of them are to be taken by a' and b' coefficients, respectively.
    // The third one is the gcd(a, b).

    // todo check for safety - should be safe
    let mut state = (1, -vec.last().unwrap().2);

    // The algebra here is in its minimal form.
    for i in vec.iter().rev().skip(1) {
	// update our current state with the new row in a table, as we are going up.
	let u = state.0;
	let v = state.1;
	state.0 = v;
	state.1 = - v * i.2 + u;
    }

    state
}

fn main() {
    let mut buf = String::new();

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


    let (mut a, mut b): (i32, i32) = (-1, -1);
    io::stdin().read_line(&mut buf);
    let data: Vec<i32> = buf
	.split_whitespace()
	.map(
	    |x| { x.parse::<i32>().unwrap() }
	).collect();
    a = data[0];
    b = data[1];

    match matches.value_of("mode") {
	Some("gcd") => {
	    println!("{}", gcd(a, b));
	},
	Some("extended_gcd") => {
	    println!("{:?}", extended_gcd(a, b));
	},
	Some("numerical_experiment") => {
	    println!("numerical_experiment");
	},
	_ => {}
    }

    /*
     * 1) d = gcd(a, b) ? simple gcd
     * 2) au + bv = d ? what are u and v? extended gcd
     * 3) as + bt = c ? if c is given. not gcd(a, b), but given.
     */

    let (a, b): (i32, i32);
    // read_line_of_vars!(i32, lines, a, b);
}
