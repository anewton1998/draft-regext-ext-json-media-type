# About

This code demonstrates a simple RDAP server and client using
the `application/extrdap+json` media type.

# Building

This code is written in Rust. If you do not have Rust installed,
visit [rustup.rs](https://rustup.rs).

For most systems, installation will be:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or use your systems package manager.

Once you have Rust installed, do the following:

```
cargo build  
```

# Running

In a terminal window, run this command to start the server(s):

```
cargo run --bin servers
```

Then, in another terminal run this command to run the client:

```
cargo run --bin client
```
