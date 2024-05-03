// quicksort is based on the divide and conquer algorithm, it picks a pivot and partitions the array around the pivot.

// different forms of quicksort

// Always pick the first element as a pivot
// Always pick the last element as a pivot
// Pick a random element as a pivot
// Pick median as a pivot

// there are three parts to the algorithm, if the array is less than 1 then return,
// 2 recursive calls on the left and right partitions.

// Another divide-and-conquer algorithm that selects a pivot element and partitions the array into two subarrays
// such that elements smaller than the pivot are on the left and larger on the right. It then recursively sorts the subarrays.
// It has a time complexity of O(n log n) on average and is widely used due to its efficiency.

fn quicksort(array: &mut [i32]) {
    if array.len() <= 1 {
        return;
    }

    let pivot_index = partition(array);
    let (left, right) = array.split_at_mut(pivot_index);

    quicksort(left);   // recursively  on both left and right hand side
    quicksort(&mut right[1..]);   // right hand side needs to be mutable.
}


// partitioning
//

fn partition(array: &mut [i32]) -> usize {

    let pivot_index = array.len() / 2;   // split in the middle/median
    let last_element =  array.len() - 1;

    array.swap(pivot_index, last_element);     // swaps the median value with the last in the array.  array.len() - 1 since array index starts on 0
    let mut i = 0;                           // create a counter to start the index of the resultant array at zero
    for j in 0..last_element {         // iterate through the un-sorted array,
        if array[j] <= array[last_element] {       // re-arrange based on size of element compared to the last
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, array.len() - 1);
    i
}

fn main() {
    let mut array = vec![4, 2, 7, 2, 9, 3, 5];

    println!("Original array: {:?}", array);
    quicksort(&mut array);
    println!("Sorted array: {:?}", array);
}