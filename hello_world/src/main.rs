fn main() {
	let x: i32 = 5;

    println!("The value of x is: {}", x);
    println!("Calling add_one: {}", add_one(1));

    // Arays
    let mut a = [0; 20];
    a[2] = 3;
    a[3] = 4;
    a[4] = 5;
    println!("\nArrays\na has {} elements", a.len());
    let middle = &a[1..6];
    println!("{:?}", middle);

    // Tuples
    let tuple = (1, 2, 3, 4);
    println!("\nTuples\nHere is some values{:?}", tuple);

    // if statements are cool
    let y = if x == 5 {
    	10
    } else {
    	15
    };

    println!("\nSome cool loops", );
    loops();

    // vec
    let v = vec![1, 2, 3, 4];
    let i: usize = 0; // v[i]; works
    let j: i32 = 0; // v[j]; dosen't

    // Borrowing
    // Rules
    //	1) Borrow must last for a scope, no greate than that of the owner
    //	2) have one or the other of two kinds of borrow, not @ same time
    //		a} one or more reference (&T) to a resource
    //		b} exactly one mut refernce (&mut T)
    println!("\nBorrowing");
    let v1 = vec![1, 2, 3];
 	let v2 = vec![1, 2, 3];
 	let answer = foo(&v1, &v2);
 	println!("{:?}", answer);
       
    // Lifetimes
	// Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
	// If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.
	// If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.
    fn bar<'a>(x: &'a i32) {

    }
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
	// do stuff

	// return the answer
	42
}

fn print_sum(x: i32, y: i32) {
	println!("The sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
	x + 2;
	x + 1
}

fn loops() {
	for (i,j) in (5..10).enumerate() {
    	println!("i = {} and j = {}", i, j);
	}
}