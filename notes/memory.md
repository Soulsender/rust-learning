# Memory

## Pointers
A pointer is a value that indicates a location in memory.

Pointers can be made in different ways, such as allocated memory in the heap.

## The Stack

Variables live in *frames*. A frame is a mapping from variables to values in a single scope. 

The stack holds data for a function, while the heap holds data that outlives a function.

## The heap
The heap is a region of memory where data can live indefinitely. 

Heap data is not tired to a specific stack frame.

### **EXAMPLE A:**

```rs
fn main() {
    let a = [0; 1_000_000];
    let b = a;
}
```
Copying this costs a lot of memory. These are now two arrays.

Visual memory:

```
a -----> 1000000...
b -----> 1000000...
```

The `Box` construct can put data on the heap by copying the pointer for `a` to `b`.

```rs
fn main() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
}
```
This stops a large amount of memory from being used, and now there is only one array instead of two.

Visual memory:

```
a ---> 1000000...
b -----^
```

### **EXAMPLE B:**
```rs
fn main() {
    let a = Box::new(15);
    let b = a;
    let c = Box::new(15);
}
```
The memory ends up looking like this:
```
a ------> 15
b --------^

c ------> 15
```
where `a` and `b` point to the same piece of memory, and `c` points to a different piece of memory.