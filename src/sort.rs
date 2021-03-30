use std::cmp::Ordering;

use rand::Rng;

pub fn top_n<T, F: Fn(&T, &T) -> Ordering>(v: &mut [T], n: usize, f: &F) {
    if v.len() <= 1 {
        return;
    }
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..v.len());
    v.swap(0, r);
    //qsort pivot
    let mut back = v.len() - 1;
    let mut i = 0;
    while i < back {
        match f(&v[i], &v[i + 1]) {
            Ordering::Greater => {
                v.swap(i, i + 1);
                i += 1;
            }
            _ => {
                v.swap(i + 1, back);
                back -= 1;
            }
        }
    }
    top_n(&mut v[..i], n, f);
    if i < n && i + 1 < v.len() {
        top_n(&mut v[(i + 1)..], n - i, f);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_top_n() {
        let mut v = vec![3, 5, 3, 8, 65, 8, 5, 3, 4, 5, 6, 0, 12, 33, 54, 34];
        top_n(&mut v, 5, &|&a, &b| b.cmp(&a));
        assert_eq!(&v[..5], &vec![65, 54, 34, 33, 12]);
    }
}
