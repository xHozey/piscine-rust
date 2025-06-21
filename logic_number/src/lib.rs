pub fn number_logic(num: u32) -> bool {
    let mut cl = num;
    let mut compare = 0;
    while cl > 0 {
        compare += (cl%10).pow(num.to_string().len() as u32); 
        cl /= 10
    }

    compare == num
}