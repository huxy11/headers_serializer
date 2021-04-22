## 
A handy tool specializing in serializing Rust data structures into HashMap(maybe more if necessary) which is based on Rust's #[derive] mechanism, just like what you would use to automatically derive implementations of the built-in Clone, Copy, Debug, or other traits. It is able to generate implementations for most structs as long as all the fields(or wrapped value, for options) have the to_string() methods.

## Usage
add serialize_to_maps = {version = "0.1", path = ""}
