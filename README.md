# memoria

A database with simplicity.

## Features

1. Written in Rust.
1. Memory -> SSD -> HDD, three level storage.
1. Embedded, executed in one thread.
1. Support all Rust std collection types.
1. Support time dimension migrations.
1. Implement data and program fusion.

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
enum Gourp {
    Architect (val = "architect"),
    Developer (val = "developer"),
    Designer (val = "designer"),
}
```

`role.memoria`

```rust
enum Role {
    DataViewer (val = "data viewer"),
    DataModifier (val = "data modifier"),
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
pub enum Group {
    #[variant(val = "architect")]
    Architect,
    #[variant(val = "developer")]
    Developer,
    #[variant(val = "designer")]
    Designer,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub enum Role {
    #[variant(val = "data viewer")]
    DataViewer,
    #[variant(val = "data modifier")]
    DataModifier,
}
```

### Data operations

```rust
let mut u = User {
    id: memoria::id(),
    name: "clia".to_owned(),
    age: 24,
    gender: Gender::Male.ref(),
    group: Some(Group::Architect.ref()),
    roles: vec![Role::DataViewer.ref(), Role::DataModifier.ref()],
};
u.sync().await?;

u.age = 36;
u.sync().await?;
```

### Data queries

```rust
let v: Vec<User> = memoria::find_vec(|r| r.name == "clia").await?;

for u in v {
    println!("user clia's age: {}, gender: {}", u.age, u.gender.val().await?);
}
```
