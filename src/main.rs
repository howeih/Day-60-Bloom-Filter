extern crate rand;
extern crate typenum;

use std::collections::HashSet;

use bit_array::BitArray;
use rand::Rng;
use typenum::U1000000;

const TEST_SIZE: usize = 100000;

struct BloomFilter {
    array: BitArray<usize, U1000000>,
    k: usize,
}

impl BloomFilter {
    fn ihash(x: &[usize], h: &mut usize) -> usize{
        for i in x {
            *h = ((*h + *i) * 127733) % (1 << 32);
        }
        *h
    }

    fn hash(&self, x: &[usize]) -> Vec<usize> {
        let mut hash = Vec::<usize>::new();
        let mut h = 86813;
        for _ in 0..self.k {
            let hash_value = BloomFilter::ihash(&x, &mut h);
            hash.push(hash_value % self.array.len() as usize)
        }
        hash
    }

    fn add(&mut self, x: &[usize]) {
        for h in self.hash(x) {
            self.array.set(h as usize, true);
        }
    }

    fn contains(&self, x: &[usize]) -> bool {
        let mut contains = true;
        for i in self.hash(&x) {
            contains &= self.array.get(i as usize).unwrap();
        }
        contains
    }

    fn new(k: usize) -> Self {
        BloomFilter {
            array: BitArray::<usize, U1000000>::new(),
            k,
        }
    }
}

fn gen_test_set() -> HashSet<Vec<usize>> {
    let mut test_set = HashSet::new();
    let mut rng = rand::thread_rng();
    for _ in 0..TEST_SIZE {
        let value = vec![rng.gen_range(0, 256), rng.gen_range(0, 256), rng.gen_range(0, 256), rng.gen_range(0, 256)];
        test_set.insert(value);
    }
    test_set
}

fn get_diff_test_set(a: &HashSet<Vec<usize>>) -> HashSet<Vec<usize>> {
    let mut test_set = HashSet::new();
    let mut rng = rand::thread_rng();
    for _ in 0..TEST_SIZE {
        let value = vec![rng.gen_range(0, 256), rng.gen_range(0, 256), rng.gen_range(0, 256), rng.gen_range(0, 256)];
        if !a.contains(&value) {
            test_set.insert(value);
        }
    }
    test_set
}


fn measure_accuracy(a: &HashSet<Vec<usize>>, b: &HashSet<Vec<usize>>, k: usize) {
    let mut filter = BloomFilter::new(k);
    let mut fp = 0;
    for i in a {
        filter.add(i);
    }

    for x in b {
        if filter.contains(x) {
            fp += 1;
        }
    }
    let acc: f32 = 1. - fp as f32 / b.len() as f32;
    println!("{} hashes, {} false positives, {:.4} accuracy", k, fp, acc);
}

fn main() {
    let a = gen_test_set();
    let b = get_diff_test_set(&a);
    for &k in &[1, 2, 3, 4] {
        measure_accuracy(&a, &b, k);
    }
    for &k in &[1, 2, 3, 4, 6, 8] {
        measure_accuracy(&a, &b, k);
    }
}
