// Algorithm (insertion sort) to sort an input array of numbers in place
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}

fn main() {
    let mut v = vec![5, 2, 4, 6, 1, 3];
    insertion_sort(&mut v);
    println!("{:?}", v)
}
