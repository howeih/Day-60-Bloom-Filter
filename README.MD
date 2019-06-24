Day 60: Bloom Filter
<br>
Bloom filter is a probabilistic data structure based on hashing. It’s very similar to hash table, but differs in several important aspects.
<br>
only add() and contains() operations are supported
<br>
contains() may return false positives
<br>
uses fixed memory size (can’t enlarge), but scales well for big data
<br>
It is obvious that any item that has been added will be correctly reported as present. However, it may happen that items which have not been added will be reported as present, too. That’s a false positive.
<br>
run:
```

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


```
result:
```

1 hashes, 9469 false positives, 0.9053 accuracy
2 hashes, 4642 false positives, 0.9536 accuracy
3 hashes, 2312 false positives, 0.9769 accuracy
4 hashes, 3429 false positives, 0.9657 accuracy
1 hashes, 9469 false positives, 0.9053 accuracy
2 hashes, 4642 false positives, 0.9536 accuracy
3 hashes, 2312 false positives, 0.9769 accuracy
4 hashes, 3429 false positives, 0.9657 accuracy
6 hashes, 3392 false positives, 0.9661 accuracy
8 hashes, 6677 false positives, 0.9332 accuracy


```