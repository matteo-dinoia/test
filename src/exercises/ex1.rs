use std::collections::HashMap;

pub fn test_all(){
    println!("{}", string_reverse("ciao"));
    println!("{}", bigger(10, 20));
    println!("{}", multiply(30, 2.0, 3000000000000000f64));
    println!("{}", e_equals_mc_squared(3.0f32));
    println!("{:?}", min_max(vec!(1,2,3,7,-3,4,5)));
    println!("{:?}", min_max(vec!()));
    println!("{:?}", lord_farquaad(
        &"globally-defined constant containing the value".to_string()));
    
    let mut furnitures: HashMap<String, f32> = HashMap::new();
    furnitures.insert("wardrobe".to_string(), 10.9);
    furnitures.insert("desk".to_string(), 12.4);
    println!("{:?}", furniture_borrow(&furnitures, &"wardrobe".to_string()));
    println!("{:?}", furniture_borrow(&furnitures, &"none".to_string()));
    
    let text = "cane".to_string();
    let text_appended = append(&text);
    println!("{} {}", text, text_appended);

    println!("{} {} {} {}", is_armstrong(9), is_armstrong(10),
             is_armstrong(153), is_armstrong(154));
    
    let mat = ((1,2), (3,4));
    println!("{:?} {:?}", mat, transpose(&mat));
}

fn string_reverse(to_rev: &str) -> String{
    let mut ret = String::new();
    let mut iter = to_rev.chars();
    //chars has a rev method
    // s.chars().rev().collect()
    // string as pop method
    for i in 0..to_rev.len() {
        ret.push(iter.next_back().unwrap());
    }

    ret
}

fn bigger (a: i32, b:i32) -> i32{
    if a >= b {a}
    else {b}
}

fn multiply(a: i32, b: f32, c: f64) -> f64{
    (a as f64) * (b as f64) * c
}

pub const C_SPEED: f32 = 300000000f32;

fn e_equals_mc_squared(mass: f32) -> f32{
    mass * C_SPEED.powi(2)
}

fn min_max(array: Vec<i32>) -> Option<(i32, i32)>{
    let mut min = *array.get(0)?;
    let mut max = min;
    for el in array {
        if el > max {
            max = el;
        }else if el < min {
            min = el;
        }
    }
    
    Some((min, max))
}

fn lord_farquaad(some_str: &String) -> String{
    let mut ret = String::new();
    for mut el in some_str.chars() {
        if(el == 'e') { el = '💥' }
        ret.push(el);
    }

    ret
}

fn furniture_borrow(furniture: &HashMap<String, f32>, name_to_find: &String) -> f32{
    match furniture.get(name_to_find) {
        Some(price) => *price,
        _ => -1.0,
    }
}

fn append(a: &String) -> String{
    let mut ret = a.clone();
    ret.push_str("foobar");
    
    ret
}

fn is_armstrong(number: i32) -> bool{
    let mut digits = Vec::new();
    
    let mut n = number;
    while n != 0 {
        let digit = n % 10;
        digits.push(digit);
        
        n = n / 10;
    }
    
    let len = digits.len() as u32;
    let sum : i64 = digits.iter().map(|&a| a.pow(len) as i64).sum();
    
    
    sum == number as i64
}

fn transpose(matrix: &((i32, i32), (i32, i32)) ) -> ((i32, i32), (i32, i32)){
    ((matrix.0.0, matrix.1.0), (matrix.0.1, matrix.1.1))
}