# memoria

A database with simplicity.

## Features

1. Written in Rust.
1. Memory -> SSD -> HDD, three level storage.
1. Embedded, executed in one thread.
1. Support all Rust std collection types.
1. Support time dimension migrations.

## Examples

### Define data structures

```rust
use memoria::{Data, Ref, field, variant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct User {
    #[field(UNIQUE, MAX_LEN=20)]
    pub name: String,
    #[field(MAX_VAL=200)]
    pub age: u16,
    pub gender: Ref<Gender>,
    pub group: Option<Ref<Group>>,
    #[field(MIN_LEN=1)]
    pub roles: Vec<Ref<Role>>,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub enum Gender {
    #[variant(VAL="male")]
    Male,
    #[variant(VAL="female")]
    Female,
    #[variant(VAL="other")]
    Other,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Group {
    #[field(UNIQUE)]
    pub name: String,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Role {
    #[field(UNIQUE)]
    pub name: String,
}
```

### Data operations

```rust
let g = Group {
    name: "creator".to_owned(),
};
memoria::save_or_update(g)?;

let r1 = Role {
    name: "data viewer".to_owned(),
};
memoria::save_or_update(r1)?;

let r2 = Role {
    name: "data modifier".to_owned(),
};
memoria::save_or_update(r2)?;

let u = User {
    name: "clia".to_owned(),
    age: 24,
    gender: Gender::Male.ref(),
    group: Some(g.ref()),
    roles: vec![r1.ref(), r2.ref()],
};
memoria::save(u)?;

let mut u: User = memoria::get_mut(u.memoria_id()).await?;
u.age = 36;
memoria::save(u)?;
```

### Data queries

```rust
let v: Vec<User> = memoria::find_vec(|r| r.name == "clia").await?;

if v.len() > 0 {
    let u = v[0];
    println!("user clia's age: {}, gender: {}", u.age, u.gender.val());
}
```
