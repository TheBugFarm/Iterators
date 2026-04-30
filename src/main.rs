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

fn main() {
    let a = vec![1, 2, 3, 4, 5, 56, 88, 684987631];
    let v = a.iter();
    println!("{:?}",a);
    println!("{:?}",v);
    for val in v {
        if val % 2 != 0 {
            println!("{}", val);
        }
    }
}