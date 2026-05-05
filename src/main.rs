use std::collections::HashMap;
fn main() {
    
    let mut scores = HashMap::new();
    scores.insert("bitch", 88);
    scores.insert("ninni", 24);
    scores.insert("green", 368);
    scores.insert("prepaid", 654);
    println!("{:?}", scores);

    for (key, value) in scores.iter() {
        println!("{} {}", key, value);
    }

    for (key, value) in scores.iter_mut() {
        *value += 10;
        println!("{} {}", key, value);
    }

}
/*fn main() {
    let v = vec![1, 2, 3, 4,];
    let y = v.iter();
    let r: Vec<i32> = y.filter(|x| **x % 2 == 0).map(|x| x*x).collect();
    println!("{:?}", r);
}*/
/*fn main() {
    let v = vec![2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", v);
    let r = v.iter();
    let y = r.filter(|x| **x % 2 == 0);
    for i in y {
        println!("{}", i);
    }

}*/
/*fn main() {
    let v = vec![1, 2, 3];
    let r = v.iter();
    let y = r.map(|x| x % 2);
    for i in y {
        print!("{}", i);
    }
}*/
/*fn main() {
    let v = vec![1, 2, 3, 78];
    let r = v.iter();
    let sum: i32 = r.sum();
    println!("{}", sum);
}*/
/*fn main() {
    let v = vec![1, 2, 3];
    let r = v.into_iter();
    println!("{:?}", r);
    //println!("{:?}", v);
}*/
/*#![allow(unused_variables)]
fn main()
{
    let l = vec![1, 2, 3];
    let mut t = l.iter();
    let first = t.next();
    let second = t.next();
    let third = t.next();
    println!("{:?}", first);
    println!("{:?}", second);
    println!("{:?}", third);

    //while let Some(val) = t.next()
    //{
        //println!("{}", val);
    //}
}*/
/*#![allow(unused_variables)]
fn main()
{
    let mut l = vec![354, 384, 3, 3557,5];
    let t = l.iter_mut();
    println!("{:?}", t);

    for val in t
    {
        *val += 1;
        println!("{}", val)
    }
}*/

/*fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 56, 88, 684987631];
    let v = a.iter_mut();
    println!("{:?}",v);
    for val in v {
        *val = *val + 1;
    }
}*/
