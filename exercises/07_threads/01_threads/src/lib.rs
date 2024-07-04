// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawn threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.len() == 0 {
        return 0; // 我一开始忽视了len==0的情况，导致后面split_at()时panic
    }
    if v.len() == 1 {
        return v[0];
    }
    let (v1, v2) = v.split_at(v.len() / 2);
    // 20240704
    // 此时v1 v2都是引用了v，无法在spawn内使用，需要先clone
    // let v1 = v1.clone(); 不行，只是复制引用，v1还是v1。!!这里有点反直觉吧??
    // let v1 = &*v1; // 不行，v1还是v1
    // let v1 = &(*v1).clone(); 不行，[i32]没有clone方法，编译不过
    // let v1 = &(*v1.clone()); 不行，还是复制引用，没有意义
    // 总结：spawn不接受引用，必须给他一个值；[i32]是!Sized是无法传递的值，也不被接受。
    // 总结：看spawn参数定义，F: FnOnce() -> T + Send + 'static。引用不满足'static, [i32]不满足Send。
    // 那怎么克隆&[i32]呢，可以转换成Vec或Box。
    // 前面提到反直觉的问题，为什么&i32可以克隆而&[i32]不能被克隆呢？
    //   我感觉Rust里面的逻辑是，如果T:Clone，t:&T，则t.clone()返回T，否则t.clone()返回&T。
    //   即编译器使用`impl Clone for T`优先级高于`impl<T: ?Sized> Clone for &T`。
    //   rustc输出可以反证我上述推论："note: `A` does not implement `Clone`, so `&A` was cloned instead"。
    //   在本案中，[i32]不满足:Sized，因而不可能实现Clone，故&[i32]只能复制引用。这样解释就通了。
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();
    let t1 = thread::spawn(move || v1.iter().sum::<i32>());
    let t2 = thread::spawn(move || v2.iter().sum::<i32>());
    t1.join().unwrap() + t2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
