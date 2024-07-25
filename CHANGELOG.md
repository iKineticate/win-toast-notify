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