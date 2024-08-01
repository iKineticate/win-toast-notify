## 0.1.5 => 0.1.6
```rust
pub struct Action<'a> {
    pub activation_type: ActivationType,
    pub action_content: &'a str,
    pub arguments: &'a str,
    pub image_url: Option<&'a str>,
}
👇
pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: String,
    pub arguments: String,
    pub image_url: Option<String>,
}
```

## 0.1.4 => 0.1.5
```rust
// modify❗
pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: &'static str,
    pub arguments: String,
    pub image_url: Option<String>,
}
👇
pub struct Action<'a> {
    pub activation_type: ActivationType,
    pub action_content: &'a str,
    pub arguments: &'a str,
    pub image_url: Option<&'a str>,
}

pub struct Progress {
    pub tag: &'static str,
    pub title: &'static str,
    pub status: &'static str,
    pub value_string: &'static str,
}
👇
pub struct Progress {
    pub tag: &'a str,
    pub title: &'a str,
    pub status: String,
    pub value_string: String,
}
```
```rust
// Fix errors in examples code
// Fix doctest code in library
```


## 0.1.3 => 0.1.4
```rust
// modify❗
pub fn set_notf_open()
👇
pub fn set_open()

pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: &'static str,
    pub arguments: &'static str,
    pub image_url: Option<&'static str>,
}
👇
pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: &'static str,
    pub arguments: String,
    pub image_url: Option<String>,
}
```

```rust
// feat
pub fn set_scenario()
pub fn set_progress()
pub fn progress_update()
pub fn progress_complete()
```