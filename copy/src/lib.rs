pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}
pub fn str_function(a: String) -> (String, String) {

    let mut str_res = String::new();
    let vec_str = a.split(" ");
    for word in vec_str {
        match word.parse::<f64>() {
            Ok(val) => {
                str_res.push_str(&val.exp().to_string());
                str_res.push_str(" ");
            },
            Err(_) => (),
        }
    }
    str_res.pop();
    (a, str_res)

}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut float_vec:Vec<f64>  = vec![];

    for num in b.clone(){
        print!("{}", num);
        float_vec.push(((num as f64).abs()).ln())
    }

    (b, float_vec)
}

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}


// And its output:

// $ cargo run
// (0, 1.0, -inf)
// ("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")
// ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003])
// $