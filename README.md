# EasyLazy - Lazy initialization made easy

Looking at the available options for lazy initialization, I found that the most
required jumping through odd, unnecessary hoops.  EasyLazy aims to make lazy
initialization of a variable lightweight and easy.

EasyLazy has only 3 requirements:

- T must implement Clone
- T must implement Default with a cheap operation
- The Lazy variable must be mutable so that it can be initialized

## Usage

``` Rust
use easylazy::Lazy;
let mut my_lazy_variable = Lazy::new(Box::new(|| 10));
// my_lazy_variable is uninitialized here
assert_eq!(my_lazy_variable.get(), 10);
my_lazy_variable.get_mut() = 20;
assert_eq!(my_lazy_variable.get(), 20);
```
