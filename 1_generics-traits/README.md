# Implementing (a simplistic) MyIterator

An iterator is a struct that implements a Trait with a method called `next()` which returns an `Option` value that can contain an `Item` of a collection iterated over.

```rust
trait MyIterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

All data structures are prefixed with `My` to avoid confusions with the real implementation of Iterator.

The goal of this exercise is to implement the following methods on MyIterator trait:
- filter(...) -> MyFilter
- map(...) -> MyMap
- sum(...) -> i32

## Implement a function that can print a `MyIterator`

The first step is to print the values contained in a MyIterator which going to print all values, separated by `,` which will allow us to visualise our Iterators.

## Implement MyFilter iterator

Next we would like to create a Filter iterator given the current instance of a MyIterator. Uncomment the next two lines in the `main` function and the lines of `my_filter` method in MyIterator trait.

A _Filter_ is an iterator that wraps another iterator. Its next method returns the previous iterator next item only if this item is satisfying a predicate closure: `my_iterator.filter(|&item| item % 2 == 0);`

=> Implements the method `my_filter()` on `MyIterator` that take ownership of the current instance of `MyIterator` into a `MyFilter` iterator.

## Implement MyMap iterator

Next we would lik to create a Map iterator given then current instance of MyIterator. Uncomment the next two lines in the `main` function and the lines of `my_map` method in MyIterator trait.

A _Map_ is an iterator that wraps another iterator. Its next methods returns the previous iterator next item by applying a mapper function to the item and returns its result: `my_iterator.map(|item| item * 2);`

=> Implements the method `my_map()` on `MyIterator` that take ownership of the current instance of `MyIterator` into a `MyMap` iterator.

## Implement sum on MyIterator

Finally, we would like to implement a sum method on the iterator, for simplicity we only want the sum to be called on an iterator of `i32` values.

Uncomment and implement the lines related to the `my_sum` method in `main` function and in the MyIterator trait.

=> Implement the method `my_sum()` on `MyIterator`

At this staged, s

## Complements

- to accept a closure as parameter, use a generic type
    ```rust
    fn use_closure<F: Fn(String) -> bool>(exec: F) {
    ```
- to execute a closure from a struct field
    ```rust
    fn closure_from_struct(some_struct: SomeStruct) {
        if (some_struct.exec)(String::new()) {
            println!("Closure returned true");
        }
    }
    ```
