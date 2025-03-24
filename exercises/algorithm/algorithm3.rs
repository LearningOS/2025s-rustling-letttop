/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::Ord;
use std::fmt::Debug;

fn sort<T: Copy + Ord + Debug>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let base = array[array.len() / 2];
    let mut left_index = 0;
    let mut right_index = array.len() - 1;
    loop {
        while array[left_index] < base {
            left_index += 1;
        }
        while array[right_index] > base {
            if right_index == 0 {
                break;
            }
            right_index -= 1;
        }
        if left_index >= right_index {
            break;
        }
        //
        array.swap(left_index, right_index);
        // println!(
        //     "交换后 left_index: {}, right_index: {}，当前数组状态: {:?}",
        //     left_index, right_index, array
        // );
        left_index += 1;
        if right_index > 0 {
            right_index -= 1;
        }
    }
    sort(&mut array[..left_index]);
    sort(&mut array[left_index..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
