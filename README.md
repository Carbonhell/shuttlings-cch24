# Shuttlings-cch24

This project aims to solve [Shuttle's Christmas Code Hunt 2024 challenges](https://www.shuttle.dev/cch), to improve my
knowledge of Rust and to explore Shuttle for future hobby projects.

# Notes

In the following sections, I'll write down some useful information I learned thanks to each challenge.

## Challenge -1

Thanks to the weird enumeration of the challenges, I've cleared up some doubts I had about the module system of Rust.
Rust offers two ways to structure a project in modules:

- **mod.rs**: create a new folder, add a mod.rs file inside it importing the sibling files, and add a "mod
  _module_name_" statement in your main.rs file.
- **folder_name.rs**: create a new folder with some source code files, and create a **folder_name.rs** file as a sibling
  of **main.rs**.

ight now, the latter is what is recommended by
the [official book](https://doc.rust-lang.org/rust-by-example/mod/split.html), although it does seem to lead to
pollution of the src
folder. Something I like doing often is looking at what convention big projects follow. By looking at
what [Bevy](https://github.com/bevyengine/bevy) does, it seems like the mod.rs convention is still in use.
Whereas in most cases it seems to be mostly about a personal preference, for this challenge the usage of the mod.rs
convention
allows defining the module **challenge_-1**, which contains a hyphen in the name. Hyphens cannot be used as module
names.
By using the mod.rs convention, one can write the following code:

```rs
#[path = "challenge_-1/mod.rs"]
mod challenge_neg1;
```

In this way, the module can have a name that is different than what the file hierarchy describes.

### Resources

- https://www.reddit.com/r/rust/comments/18pytwt/noob_question_foomodrs_vs_foors_foo_for_module/

## Challenge 2

This challenge wasn't particularly complex, also thanks to the tips offered regarding which struct (Ipv4Addr) and which
operation (overflowing_add) to use.
Still, one particular detail stood out to me: to iterate the octets of the IPs given as input, I used iterators.
In particular, the following code performs the operation required to get the result from the two inputs:

```rs
let from_octets = query_params.0.from.octets();
let key_octets = query_params.0.key.octets();
let encrypted_octets: [u8; 4] = from_octets
  .into_iter()
  .zip(key_octets)
  .map(|(from_octet, key_octet)| { from_octet.overflowing_add(key_octet).0 })
  .collect::<Vec<u8>>()
  .try_into()
  .unwrap(); // This unwrap is safe due to the type of encrypted_octets depending on the type of from/key octets
Ipv4Addr::from(encrypted_octets).to_string()
```

This code is for the first task, but the other tasks are pretty much the same, with slight variations.
Two parts in particular are meaningful for me:

- the iterator functions used to process the information in the way I want (zip, map, collect), which isn't particularly
  noteworthy but getting used to it really makes me enjoy working with iterators;
- the try_into() call, which allows type conversion from vecs to array. It is required due to the fact From is
  implemented for Ipv4Addr for u8 arrays of length 4.

## Challenge 5

### Logging

For this challenge, I started looking at how to add structured logging, mainly to debug the code I wrote but also to
understand how proper logging works with a Shuttle app.
Luckily, there's a useful blog post about this on
the [Shuttle blog](https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust).
I've already used tracing in the past, although for [profiling](https://docs.rs/tracing-tracy/latest/tracing_tracy/)
instead of logging.

### Returning various type of responses with Axum

You can define an endpoint handler as a function that returns `impl IntoResponse`. This allows returning any kind of
type that implements
this trait, such as strings, status codes, header maps and so on.

An issue arises in the case you want to return, within the same handler, two or more different types. For example,
for the first task of this challenge you must return a 204 No Content response in negative cases, and a string in
positive ones. This apparently requires you to manually call `into_response()` on the value you're returning (
see [the docs](https://docs.rs/axum/latest/axum/response/index.html#returning-different-response-types). It adds
some
boilerplate, which I dislike, but I understand why it may be required.

### Serde YAML

The most popular library for YAML parsing appears to
be [deprecated](https://github.com/dtolnay/serde-yaml/releases/tag/0.9.34). The reasonable alternative seems to
be [this](https://github.com/sebastienrousseau/serde_yml), but it doesn't look as popular.

### Resources

- https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust
- https://www.shuttle.dev/blog/2023/09/20/logging-in-rust