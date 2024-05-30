pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1],  //supp
                    std::cmp::min(dp[i - 1][j],//replac
                    dp[i][j - 1]), //insert
                );
            }
        }
    }

    dp[m][n]
}

fn main() {
	let source = "alignment";
	let target = "assignment";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
}
// And its output:

// $ cargo run
// It's necessary to make 2 change(s) to alignment, to get assignment
// $
