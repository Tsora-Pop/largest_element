fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![14,64,37,82,39];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![25,54,27,91,10];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','c','d','w'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
