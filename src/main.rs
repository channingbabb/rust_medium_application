// quick sort application
// Written by: Channing Babb

use rand::Rng;
/// Sorts a mutable slice of integers in ascending order using the quicksort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable slice of integers to be sorted.
///
fn quicksort(arr: &mut [i32]) {
    let mut stack = Vec::new(); // create a new mutable vector
    let left = 0; 
    let right = arr.len() - 1; // get the length of the array and subtract 1

    stack.push((left, right));

    while let Some((left, right)) = stack.pop() { // Some = not null, pop = remove last element
        if right <= left { // if right is less than or equal to left, continue
            continue;
        }

        let pivot = partition(&mut arr[left..=right]); // partition the array around the pivot element... reference to the array from left to right

        if pivot > 0 {
            stack.push((left, left + pivot - 1)); // push the left and pivot - 1 to the stack if pivot is greater than 0
        }

        stack.push((left + pivot + 1, right)); // push the left + pivot + 1 and right to the stack
    }
}

/// Partitions a mutable slice of integers around a pivot element.
///
/// # Arguments
///
/// * `arr` - A mutable slice of integers to be partitioned.
///
/// # Returns
///
/// The index of the pivot element after partitioning.
///
fn partition(arr: &mut [i32]) -> usize { // usize = unsigned integer
    let len = arr.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    arr.swap(pivot_index, last_index); // swap the pivot index and last index

    let mut store_index = 0; // store mutable index = 0

    for i in 0..last_index { // for i in 0 to last_index
        if arr[i] < arr[last_index] { // if arr[i] is less than arr[last_index]
            arr.swap(i, store_index); // swap i and store_index
            store_index += 1;
        }
    }

    arr.swap(store_index, last_index); // swap store_index and last_index

    store_index // return store_index (no return keyword or semicolon needed?)
}

fn main() {
    // create array with 100,000 elements
    // mut = mutable
    let mut arr = [0i32; 100_000]; // 100,000 elements, i32 = 32 bit integer
    let mut rng = rand::thread_rng(); // random number generator, thread safe
    for i in 0..arr.len() { // for i in 0 to arr.len()
            arr[i] = rng.gen_range(0..1000); // generate random number between 0 and 1000... (..) means up to but not including
        }
    quicksort(&mut arr); // & = reference, mut = mutable
    println!("{:?}", arr); // {:?} = debug print
}
