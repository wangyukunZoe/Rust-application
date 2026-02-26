struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) //需要两个迭代器，把每个元素
        //捏到一起，并返回一个新的迭代器
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0) //把乘出来的数能被 3 整除的留下，其他的删除
        .sum();

    assert_eq!(18, sum);
}
