pub fn get_products(nums: Vec<usize>) -> Vec<usize> {
    if nums.is_empty() || nums.len()==1{
        return vec![];
    }
    let total_product: usize = nums.iter().product();
    let product_without_zero: usize = nums.iter().filter(|&&x| x != 0).product();
    let mut result = Vec::with_capacity(nums.len());
    for &num in &nums {
        if num != 0 {
            result.push(total_product / num);
        } else {
            result.push(product_without_zero);
        }
    }

    result
}