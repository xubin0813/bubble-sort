fn main() {
    let mut list = vec![1, 4, 3, 7, 2];
    bubble_sort(&mut list);
    println!("i32类型排序: {:?}", list);

    let mut list1 = vec!['e', 'f', 'a', 'z', 'A', 'C'];
    bubble_sort(&mut list1);
    println!("char类型排序: {:?}", list1);

    let mut list2 = vec![
        String::from("Alice"),
        String::from("David"),
        String::from("Tom"),
        String::from("Jack"),
    ];
    bubble_sort(&mut list2);
    println!("String类型排序: {:?}", list2);
}

// fn bubble_sort(list: &mut Vec<i32>) -> &Vec<i32> {
//     for _i in 0..list.len() {
//         for x in 0..list.len() - 1 {
//             if list[x] > list[x + 1] {
//                 list.swap(x, x + 1);
//             }
//         }
//     }
//     list
// }
fn bubble_sort<T: PartialOrd>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for x in 0..list.len() - 1 {
            if list[x] > list[x + 1] {
                list.swap(x, x + 1);
            }
        }
    }
    list
}
