use std::collections::HashMap;
fn main() {
    let mut v = Vec::new();
    let mut integers = HashMap::new();

    v.push(5);
    v.push(5);
    v.push(5);
    v.push(7);
    v.push(6);
    v.push(11);
    v.push(8);
    v.push(18);
    v.push(18);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    insertion_sort(&mut v);

    for ele in &v {
        let count = integers.entry(ele).or_insert(0);
        *count += 1;
    }

    println!("{:?}", v);
    my_seperator();

    if v.len() % 2 == 0 {
        println!(
            "median of given vector is: {}",
            ((&v[(v.len() - 1) / 2]) + (&v[v.len() / 2])) / 2
        );
    } else {
        println!("median of given vector is: {}", (&v[(v.len()) / 2]));
    }

    my_seperator();
    println!("{:?}", integers);

    let mut ans= 0;
    let mut comp = -1;
    for (_key, value) in &integers {
        if *value > comp {
            comp = *value;
            ans = **_key;
        }
    }

    my_seperator();
    println!("mode of given vector is: {}",ans);

}

fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn my_seperator() {
    let my_separator = String::from("-").repeat(60);
    println!("{my_separator}");
}
