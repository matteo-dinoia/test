use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub(crate) fn test_all() {
    println!("{} {} {} {}", is_it_luhn("ciao".to_string()),
             is_it_luhn("134".to_string()), is_it_luhn("0".to_string()),
             is_it_luhn("4539 3195 0343 6467".to_string()));

    let mut data = Parking::new();

    data.add("123456".to_string(), "Cane".to_string(), 10000);
    data.add("654321".to_string(), "Marco".to_string(), 10);
    data.add("000000".to_string(), "Luca".to_string(), 40000);
    println!("{:?}", recognise_owner(&data, "Cane".to_string()));

    test::test_point();
    
    let mut hash = HashMap::new();
    hash.insert(0, Sentence::new("Hello my name was cool yesterday"));
    hash.insert(1, Sentence::new("Hi my name is cool"));
    hash.insert(2, Sentence::new("Ciao cane"));
    println!("{:?} {:?} {:?}", test2::magic_sentence(&hash, 0, 1),
             test2::magic_sentence(&hash, 0, 3),
             test2::magic_sentence(&hash, 0, 2));
}
// missing ex 6 (boring)

#[derive(Debug)]
struct Sentence{
    words: Vec<String>,
}

impl Sentence {
    pub fn new_default() -> Self {
        Self { words: Vec::new() }
    }

    pub fn new(sentence: &str) -> Self {
        let words = sentence.split_whitespace()
            .map(|x| {x.to_string()}).collect::<Vec<_>>();
        Self { words }
    }
}

mod test2{
    use std::cmp::min;
    use std::collections::HashMap;
    use crate::exercises::ex3::Sentence;

    pub fn magic_sentence(hashmap: &HashMap<i32, Sentence>, i: i32, j: i32)
                          -> Result<Sentence, &str>{
        let a = &hashmap.get(&i).ok_or("Index i not found")?.words;
        let b = &hashmap.get(&j).ok_or("Index j not found")?.words;
        
        let mut ret = Sentence::new_default();
        
        for i in 0..min(a.len(), b.len()){
            if a[i] == b[i]{
                ret.words.push(a[i].clone());
            }
        }
        
        if ret.words.len() == 0 { Err("Empty intersection") }
        else { Ok(ret) }
    }
}

mod point{
    #[derive(Clone)]
    pub struct Point{
        pub x: f32 ,
        pub y: f32
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub fn distance(&self, o: &Point) -> f32 {
            let squared = (self.x - o.x).powi(2) + (self.y - o.y).powi(2);
            squared.powf(0.5)
        }
    }
}

mod line{
    use std::mem::swap;
    use crate::exercises::ex3::point::Point;
    pub struct Line{
        start: Point,
        end: Point,
        m: f32,
        q: f32,
    }

    impl Line {
        pub fn new(mut start: Point, mut end: Point) -> Self {
            if start.x > end.x {swap (&mut start, &mut end)}

            let m = (start.y - end.y)/(start.x - end.x);
            let q = start.y - m * start.x;
            Self { start, end, m, q }
        }

        pub fn contains(&self, p: &Point) -> Result<(), String>{
            if p.x < self.start.x || p.x > self.end.x {
                Err("Not in line (left or right)".to_string())
            } else if self.start.y + (p.x - self.start.x) * self.m == p.y {
                Ok(())
            } else { Err("Not in line (not on infinite line)".to_string()) }

        }
    }
}

mod test{
    use crate::exercises::ex3::point::Point;
    use crate::exercises::ex3::line::Line;

    pub fn test_point() {
        let p1 = Point::new(1.,2.);
        let p2 = Point::new(4.,6.);
        let p3 = Point::new(7.,10.);

        let l1 = Line::new(p1.clone(), p2.clone());
        let l2 = Line::new(p1.clone(), p3.clone());

        println!("{:?} {:?}", l1.contains(&p3), l2.contains(&p3));
    }
}


#[derive(Debug)]
struct Hour(u8, u8);

#[derive(Debug)]
struct Date(u8, u8, u16);

#[derive(Debug)]
struct BoxShipping{
    name: String ,
    barcode: String ,
    shipment_date: Date,
    shipment_hour: Hour,
}

impl Display for BoxShipping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shipment of {} (barcode: {}) in date {:02}/{:02}/{:04} at time {:02}:{:02}",
                self.name, self.barcode,
                self.shipment_date.0, self.shipment_date.1, self.shipment_date.2,
                self.shipment_hour.0, self.shipment_hour.1
        );

        Ok(())
    }
}

enum Coin{
    Cent5, Cent10, Cent20, Cent50, Eur1, Eur2,
    TooLow,
    Undetected,
}

#[derive(Eq, PartialEq, Hash)]
struct Item{
    name: String,
    cost: u32,
}

struct VendingMachine {
    coins: u32,
    items: HashMap<Item, usize>,
}

impl VendingMachine {
    pub fn new(items: HashMap<Item, usize>) -> Self {
        Self { coins: 0, items }
    }

    pub fn add_item(&mut self, item: Item, available: usize){
        self.items.insert(item, available);
    }
    pub fn insert_coin(&mut self, coin: Coin) -> Result<&str, &str>{
        self.coins += match coin {
            Coin::Cent5 => {5}
            Coin::Cent10 => {10}
            Coin::Cent20 => {20}
            Coin::Cent50 => {50}
            Coin::Eur1 => {100}
            Coin::Eur2 => {200}
            Coin::TooLow => {return Err("Cannot accept 0.01 or 0.02");}
            Coin::Undetected => {return Err("Could not verify coin");}
        };

        Ok("Total updated")
    }

    pub fn get_item_price(item: &Item) -> u32{
        item.cost
    }

    pub fn buy(&mut self, item: Item) -> Result<u32, &str>{
        if item.cost > self.coins { return Err("Not enought money"); }

        let mut quantity = self.items.get_mut(&item).ok_or("Item not present")?;

        if *quantity <= 0 { return Err("The requested item is out of stock"); }
        *quantity -= 1;

        let remainder = self.coins - item.cost;
        self.coins = 0;
        Ok(remainder)
    }

}

fn recognise_owner(p: &Parking, plate: String) -> Option<u32> {
    let car = p.get(plate)?;
    Some(car.cost)
}

#[derive(Clone)]
struct Car{
    owner: String,
    cost: u32,
}
struct Parking{
    pub plate_data: HashMap<String, Car>
}

impl Parking {
    pub fn new() -> Self {
        Self { plate_data: HashMap::new() }
    }

    pub fn add(&mut self, plate: String, owner: String, cost: u32) -> bool{
        self.plate_data.insert(plate, Car{owner, cost})
            .is_some()
    }

    pub fn get(&self, plate: String) -> Option<&Car>{
        self.plate_data.get(&plate)
    }
}

enum Fuel{Diesel, Gasoline, LPG, Methane, Electric}

enum IP{IPv4([u8; 4]), IPv6([u16; 8])}

struct Point3D{x: f64, y: f64, z:f64}

fn is_it_luhn(s: String) -> bool{
    let s = s.replace(" ", "");
    let all_digits = s.chars().all(|c| {c.is_digit(10)});

    if s.len() < 2 || !all_digits { return false; }



    let v = s.chars()
        .map(|c| {c.to_digit(10).unwrap()})
        .collect::<Vec<u32>>();

    let mut i = 1;
    let sum = v.iter().rev().map(|x| {
            let mut digit = *x;

            if i % 2 == 0 {
                digit *= 2;
                if digit > 9 { digit -= 9; }
            }
            i += 1;

            digit
        }).sum::<u32>();

    sum % 10 == 0
}