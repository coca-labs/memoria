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

#### Method 1: Using memoria files

`user.memoria`

```rust
struct User {
    id: Id,
    name: String (unique, max_len = 20),
    birth: Date (max_val = today()),
    gender: Ref<Gender>,
    group: Option<Ref<Group>>,
    roles: Vec<Ref<Role>> (min_len = 1),
}
```

`gender.memoria`

```rust
enum Gender {
    Male (val = "male"),
    Female (val = "female"),
    Other (val = "other"),
}
```

`group.memoria`

```rust
struct Gourp {
    id: Id,
    name: String (unique, max_len = 20),
}
```

`role.memoria`

```rust
struct Role {
    id: Id,
    name: String (unique, max_len = 20),
}
```

#### Method 2: Using `proc_macro`

```rust
use memoria::{Data, Ref, Id, Date, field, variant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    #[field(unique, max_len = 20)]
    pub name: String,
    #[field(max_val = today())]
    pub birth: Date,
    pub gender: Ref<Gender>,
    pub group: Option<Ref<Group>>,
    #[field(min_len = 1)]
    pub roles: Vec<Ref<Role>>,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub enum Gender {
    #[variant(val = "male")]
    Male,
    #[variant(val = "female")]
    Female,
    #[variant(val = "other")]
    Other,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Group {
    pub id: Id,
    #[field(unique, max_len = 20)]
    pub name: String,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct Role {
    pub id: Id,
    #[field(unique, max_len = 20)]
    pub name: String,
}
```

### Data operations

```rust
let g = Group {
    id: memoria::id(),
    name: "creator".to_owned(),
};
g.sync().await?;

let r1 = Role {
    id: memoria::id(),
    name: "data viewer".to_owned(),
};
r1.sync().await?;

let r2 = Role {
    id: memoria::id(),
    name: "data modifier".to_owned(),
};
r2.sync().await?;

let mut u = User {
    id: memoria::id(),
    name: "clia".to_owned(),
    age: 24,
    gender: Gender::Male.ref(),
    group: Some(g.ref()),
    roles: vec![r1.ref(), r2.ref()],
};
u.sync().await?;

u.age = 36;
u.sync().await?;
```

### Data queries

```rust
let v: Vec<User> = memoria::find_vec(|r| r.name == "clia").await?;

if v.len() > 0 {
    let u = v[0];
    println!("user clia's age: {}, gender: {}", u.age, u.gender.val().await?);
}
```
