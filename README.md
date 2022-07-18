# memoria

A database with simplicity.

## Features

1. Written in Rust.
1. Memory -> SSD -> HDD, three level storage.
1. Embedded, executed in one thread.
1. Support all Rust std collection types.

## Examples

### Define data structures

```rust
use memoria::Data;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u16,
    pub gender: Gender,
    pub group: Group,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
}
```

### Data operations

```rust
let g = Group {
    name: "creator",
};
memoria::save_unique(g, "name")?;

let r1 = Role {
    name: "data viewer",
};
memoria::save_unique(r1, "name")?;

let r2 = Role {
    name: "data modifier",
};
memoria::save_unique(r2, "name")?;

let u = User {
    name: "clia".to_owned(),
    age: 24,
    gender: Gender::Male,
    group: g,
    roles: vec![r1, r2],
};
memoria::save(u)?;

let mut u: User = memoria::get_mut(u.id()).await?;
u.age = 36;
memoria::save(u)?;
```

### Data queries

```rust
let v: Vec<User> = memoria::find_vec(|r| r.name == "clia").await?;

if v.len() > 0 {
    let u = v[0];
    println!("user clia's age: {}", u.age);
}
```
