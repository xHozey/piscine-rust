use std::collections::HashMap;

pub fn spell(n: u64) -> String {
    let mut n = n;
    if n == 0 {
        return "zero".to_string();
    }
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");
    map.insert(6, "six");
    map.insert(7, "seven");
    map.insert(8, "eight");
    map.insert(9, "nine");
    map.insert(10, "ten");
    map.insert(11, "eleven");
    map.insert(12, "twelve");
    map.insert(13, "thirteen");
    map.insert(14, "fourteen");
    map.insert(15, "fifteen");
    map.insert(16, "sixteen");
    map.insert(17, "seventeen");
    map.insert(18, "eighteen");
    map.insert(19, "nineteen");
    map.insert(20, "twenty");
    map.insert(30, "thirty");
    map.insert(40, "forty");
    map.insert(50, "fifty");
    map.insert(60, "sixty");
    map.insert(70, "seventy");
    map.insert(80, "eighty");
    map.insert(90, "ninety");
    let mut res: Vec<String> = Vec::new();
    let postfix = ["", " thousand", " million", " billion"];
    let mut i = 0;
    while n > 0 {
        let digits = n % 1000;
        let hundred = digits / 100;
        let tens = digits % 100;
        let mut buffer = String::new();
        if hundred != 0 {
            buffer.push_str(&format!("{} hundred ", map.get(&hundred).unwrap()))
        }
        if tens < 20 && tens != 0 {
            buffer.push_str(map.get(&tens).unwrap())
        } else {
            let unit = tens % 10;
            let ten = tens / 10;
            if unit > 0 && ten > 0 {
                buffer.push_str(&format!(" {}-{}", map.get(&(ten*10)).unwrap(), map.get(&unit).unwrap()))
            } else {
                if ten > 0 {
                    buffer.push_str(map.get(&(ten*10)).unwrap())
                }
                if unit > 0 {
                    buffer.push_str(map.get(&unit).unwrap())
                }
            }
        }
        buffer.push_str(postfix[i]);
        res.push(buffer.trim().to_string());
        res.push(" ".to_string());
        i += 1;
        n /= 1000;
    }
    res.reverse();
    res.concat().trim().to_string()
}
