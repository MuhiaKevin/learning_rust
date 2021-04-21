# Understanding Ownership

Rust uses two data structures: Heap and Stack to store data

https://stackoverflow.com/questions/2308751/what-is-a-memory-heap

* Stack data structure:
    - It stores data in the order it gets them and removes them in opposite order
    - All data stored in the stack must have a fixed size 

* Heap Data structure:
    - Data that has unknown size at compile time or the size might change must be stored on the heap instead


* To store data with unkonw size, you request a certain amount of size
* The memory allocator finds an empty spot in the heap that is big enough and marks it as being used and returns a pointer which is the address of that location (allocating on the heap).
* Becuse the pointer has a known fixed size, the pointer can now be store on the stack since accessing elements in the stack is faster
* When you want to access the data, you first get the pointer from the stack. The pointer will be followed to get the actual data.


## Analogy 

Think of being seated at a restaurant. When you enter, you state the number of people in your group ( you request a certain amount of size ), and the staff (memory allocator) finds an empty (find empty spot in the heap) table (heap) that fits everyone and leads you there (give a pointer to the memory location). If someone in your group comes late, they can ask where you’ve been seated to find you (Getting pointer from the stack and following the pointer to the memory location).

* Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointe to get there

* Processors are faster if they jump less in memory 
* A processor can do its job better if it works on data thats close to other data rather than farther away 

## Ownership rules

* Each value in rust has an Owner
* There can only be one owner at a time
* When the owner goes out of scope the value will be dropped


## String slices

String slices are usually references to a part of a String

Example

```
 let s = String::from("hello world"); // this is the string
 let hello = &s[0..5];
 let world = &s[6..11];

```
