# Using Structs to structure related data

They are like objects in javascript

Structs are similar to tuples in that they can be used to couple similar data

### Creating a struct
```rs
struct Lion {
    // these are fields
    name: String,
    age: u32,
    weight_in_kg: u32,
    gender: String,
}

```

### Creating an instance of a struct

```rs
 let lion1 = Lion {
        name: String::from("Morodhi"),
        age: 16,
        weight_in_kg: 180,
        gender: String::from("male"),
    };
```


### Accessing a field


```rs
    println!(
        " Lion Name: {} Age: {} weight: {} gender: {}",
        lion1.name, lion1.age, lion1.weight_in_kg, lion1.gender
    )
```

### Changing the value of an instance field

```rs
    let mut lion1 = Lion {name: "morodhi", age:"12"}
     lion1.name = String::from("Simba");   
     println!("{}", lion1.name)
```

