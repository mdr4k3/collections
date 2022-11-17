
//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.


use std::collections::HashMap;

fn main() {
    let mut list:[i32;10] = [14,22,333,357,567,621,745, 357, 83, 11];
    let middle = middle_value(&list);
    println!("Middle value of sequence is: {middle}");

    let md = mediana(&mut list);
    println!("Mediana of sequence is: {md}");

    let moda = moda(&list);
    println!("Moda of sequence is: {moda}");
}

fn middle_value(l: &[i32;10]) -> i32 {
    let v:Vec<i32> = l.to_vec();

    let mut sum: i32 = 0;

    for i in &v {
        sum += i;
    }
    let d = v.len() as i32;
    return sum / d;
}

fn mediana(l: &mut [i32;10]) -> i32 {
    l.sort();
    let v: Vec<i32> = l.to_vec();
    let mid: usize = v.len() / 2;
    let mid_value: i32 = v[mid];
    return mid_value;
}

fn moda(list: &[i32; 10]) -> i32 {
   let mut map: HashMap<&i32, i32> = HashMap::new();
   let mut max_val: i32 = 0;
   let mut max_key: i32 = 0; 

   for i in list {
    let count = map.entry(i).or_insert(0);
    *count += 1;
   }

   for (k,v) in map.iter() {
    if *v > max_val {
        max_key = **k;
        max_val = *v;
    }
   }

   return max_key;
}
