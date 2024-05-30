pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    if m > n{
        m - n + 1
    }else{
        n - m + 1
    }
}


// fn main() {
// 	let source = "alignment";
// 	let target = "assignment";
// 	println!(
// 		"It's necessary to make {} change(s) to {}, to get {}",
// 		edit_distance(source, target),
// 		source,
// 		target
// 	);
// }
// And its output:

// $ cargo run
// It's necessary to make 2 change(s) to alignment, to get assignment
// $
