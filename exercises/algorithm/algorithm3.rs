/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: std::cmp::PartialOrd + Clone>(array: &mut [T]){
	//TODO
    // 冒泡排序
    // for i in 0..array.len() - 1 {
    //     for j in (i+1)..array.len() {
    //         if array[i] > array[j] {
    //             let tmp = array[i].clone();
    //             array[i] = array[j].clone();
    //             array[j] = tmp;
    //         }
    //     }
    // }
    // 选择排序
    // for i in 0..array.len() {
    //     let mut min = i;
    //     for j in i..array.len() {
    //         if array[j] < array[min] {
    //             min = j;
    //         }
    //     }
    //     let tmp = array[min].clone();
    //     array[min] = array[i].clone();
    //     array[i] = tmp;
    // }
    //插入排序
    // for i in 1..array.len() {
    //     for j in (0..i).rev() {
    //         if array[j + 1] < array[j] {
    //             let tmp = array[j].clone();
    //             array[j] = array[j + 1].clone();
    //             array[j + 1] = tmp;
    //         }
    //     }
    // }
    //希尔排序
    let len = array.len();
    let mut gap = len / 2;
    while(gap > 0) {
        for i in gap..len {
            let tmp = array[i].clone();
            let mut j = i;
            while j >= gap && tmp < array[j - gap] {
                array[j] = array[j - gap].clone();
                j -= gap
            }
            array[j] = tmp;
        }
        gap /= 2;
    }
    //堆排序
    
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