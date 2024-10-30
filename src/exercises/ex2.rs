use std::slice::Iter;
use std::collections::HashMap;

pub fn test_all() {
    let mut arr = [1,2,3,3,5,6,7,8,9,10];
    modify_odd(&mut arr);
    println!("{:?}", arr);

    println!("{:?}", count_character("ciao come va?"));

    println!("{:?}", split_at_value(&arr, 6));
    println!("{:?}", split_at_value(&arr, 11));

    sub_slice(&vec![1,2,3,3,5,6,7,8,9,10], &vec![3,3,5,6]);
    sub_slice(&vec![1,2,3,3,5,6,7,8,9,10], &vec![3,7]);
    println!("{:?}", max(&vec![1,2,11,-3,5,6,7,8,9,10]));
    println!("{:?}", max(&vec![]));

    let mut arr2 = vec![1,2,11,-3,5,6,7,8,9,10];
    swap(&mut arr2);
    println!("{:?}", arr2);

    println!("{}", is_sorted(&vec![1,2,3,3,5,6,7,8,9,10]));
    println!("{}", is_sorted(&vec![1,2,11,-3,5,6,7,8,9,10]));

    let mut arr_str = vec!["Ciao".to_string(), "cane".to_string()];
    insert_if_longer(&mut arr_str, "ciaoooneeeeeeee".to_string());
    insert_if_longer(&mut arr_str, "ciaooee".to_string());
    println!("{:?}", arr_str);

    enum Either<T,U>{
        First(T),
        Second(U),
    };

    let arr3 = vec![Either::First("ciao".to_string()), Either::Second(10)];


    println!("{:?}", merge(&[1,3,5,7], &[2,3,4,6,10,11,12]));
    
    let exp1 = Box::new(Expression::Operation{
        left: Box::new(Expression::Number(1)), 
        op: Operation::Add,
        right: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Mul,
            right: Box::new(Expression::Number(8)),
        }),
    });

    let exp2 = Box::new(Expression::Operation{
        left: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Div,
            right: Box::new(Expression::Number(0)),
        }),
        op: Operation::Add,
        right: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Mul,
            right: Box::new(Expression::Number(8)),
        }),
    });

    let exp3 = Box::new(Expression::Operation{
        left: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Div,
            right: Box::new(Expression::Number(2)),
        }),
        op: Operation::Add,
        right: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Mul,
            right: Box::new(Expression::Number(8)),
        }),
    });

    let exp4 = Box::new(Expression::Operation{
        left: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5000000)),
            op: Operation::Mul,
            right: Box::new(Expression::Number(5000000)),
        }),
        op: Operation::Add,
        right: Box::new(Expression::Operation{
            left: Box::new(Expression::Number(5)),
            op: Operation::Mul,
            right: Box::new(Expression::Number(8)),
        }),
    });
    println!("{:?} {:?} {:?} {:?}", evaluate_expression(exp1), evaluate_expression(exp2),
             evaluate_expression(exp3), evaluate_expression(exp4));

    let mut arr = vec![15, 8, 9, 1, 78, 30, 69, 4, 10];
    pancake_sort(&mut arr);
    println!("{:?}", arr);
}

fn modify_odd(slice: &mut [i32]) {
    for el in slice {
        if *el % 2 != 0 {
            *el = 0;
        }
    }
}

fn count_character(string: &str) -> HashMap<char, i32>{
    let mut hash = HashMap::new();

    for c in string.chars() {
        let old = match hash.get(&c) {
            Some(ptr) =>  *ptr,
            None => 0,
        };

        hash.insert(c, 1 + old);
    }

    hash
}

fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32],&[i32])>{
    let first_pos = slice.iter().position(|a| *a == value)?;
    Some(slice.split_at(first_pos))
}

fn sub_slice(container: &Vec<i32>, sub: &Vec<i32>){
    let mut flag = sub.len() == 0;
    for outside in 0..container.len(){
        flag = true;
        for inside in 0..sub.len(){
            if !(outside + inside < container.len() && container[outside + inside] == sub[inside]) {
                flag = false;
                break;
            }
        }

        if flag == true { break; }
    }

    if flag {
        println!("{:?}", sub);
    }else {
        println!("Not found");
    }
}

fn max(arr: &Vec<i32>) -> Option<i32>{ max_rec(arr, 0) }
fn max_rec(arr: &Vec<i32>, pos: usize) ->Option<i32>{
    if pos >= arr.len() { return None; }

    let old_max = max_rec(arr, pos + 1);

    if let Some(old_v) = old_max {
        return Some(arr[pos].max(old_v));
    }

    Some(arr[pos])
}

fn swap(arr: &mut Vec<i32>){
    let len = arr.len();
    if(len < 2) { return; }
    arr.swap(0, len - 1);
}

fn is_sorted(arr: &Vec<i32>) -> bool{
    is_sorted_rec(arr, 0)
}

fn is_sorted_rec(arr: &Vec<i32>, pos: usize) -> bool{
    if pos >= arr.len() { return true; }

    match max_rec(arr, pos + 1) {
        Some(maximum) => {
            arr[pos] <= maximum && is_sorted_rec(arr, pos + 1)
        },
        None => true,
    }
}

fn insert_if_longer(arr: &mut Vec<String>, string: String){
    if string.len() > 10 {
        arr.push(string);
    }
}

fn build_vector(iter: Iter<i32>) -> Vec<&i32>{
    iter.collect()
}

fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32>{
    let mut p1 = 0;
    let mut p2 = 0;

    let mut ret= vec![];

    while p1 < arr1.len() || p2 < arr2.len() {
        if(p1 >= arr1.len()  || (p2 < arr2.len() && arr2[p2] <= arr1[p1])){
            ret.push(arr2[p2]);
            p2 += 1;
        }else{
            ret.push(arr1[p1]);
            p1 += 1;
        }
    }

    ret
}

enum Operation{
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression{
    Number(i32),
    Operation{
        left: Box<Expression>,
        op: Operation,
        right: Box<Expression>,
    }
}

fn evaluate_expression(exp: Box<Expression>) -> Result<i32, String>{
    match *exp {
        Expression::Number(res) => Ok(res),
        Expression::Operation{left,op,right} => {
            let a = evaluate_expression(left)?;
            let b = evaluate_expression(right)?;
            
            match op { 
                Operation::Add => a.checked_add(b).ok_or("Overflow".to_string()),
                Operation::Sub => a.checked_sub(b).ok_or("Overflow".to_string()),
                Operation::Mul => a.checked_mul(b).ok_or("Overflow".to_string()),
                Operation::Div => a.checked_div(b).ok_or("Cannot divide by 0".to_string()),
            }
        }
    }
}

fn flip(arr: &mut Vec<i32>, k: usize){ // 0..k inclusive
    let mut left = 0;
    let mut right = k;
    while left < right {
        (arr[right], arr[left]) = (arr[left], arr[right]);
        left += 1;
        right -= 1;
    }
    
    println!("T {:?} {}", arr, k);
}

fn max_index(arr: &Vec<i32>, k: usize) -> usize{// 0..k exclusive (assume not empty & k>=0)
    let mut index = 0;
    let mut max = arr[0];
    for i in 0..k{
        if arr[i] >= max{
            max = arr[i];
            index = i;
        }
    }
    
    index
}

fn pancake_sort(arr: &mut Vec<i32>){
    for x in (1..=arr.len()).rev() {
        let maxidx = max_index(arr, x);
        if maxidx != x - 1{
            if maxidx != 0{
                flip(arr, maxidx);
            }
            flip(arr, x - 1);
        }
    }
    
}
