fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fail
fn main() {
    let number_list = vec![14,64,37,82,39];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![25,54,27,91,10];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
