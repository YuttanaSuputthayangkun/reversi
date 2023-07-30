#[test]
fn iter_until() {
    let list: [Option<i32>; 4] = [Some(1), Some(2), None, None];
    let last = list
        .into_iter()
        .enumerate()
        .filter(|(_, num)| num.is_some())
        .last();
    assert_eq!(Some((1, Some(2))), last);
}

#[test]
fn skip_while() {
    let list = [1, 1, 0, 1];
    let iter = list.iter();
    let mut skip = iter.skip_while(|&x| x == &1); // ignore 1s
    assert_eq!(skip.clone().count(), 2);
    assert_eq!((skip.next(), skip.next()), (Some(&0), Some(&1))); // test if remaining elements are matched
    assert_eq!(skip.next(), None); // list should be empty
}
