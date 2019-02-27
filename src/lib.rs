//! This is a simple crate that provides a procedural macro similar to `#[test]` that will run the test as a single future on a tokio runtime.
//!
//! # Usage
//!
//! First, you must be on nightly rust as of `2019-02-15`. Add the crate to your `Cargo.toml`.
//!
//! ``` toml
//! [dependencies]
//! tokio-async-await-test = "0.1"
//! ```
//!
//! This will give you the crate but you will also need to make sure that you also have
//! `futures-preview` ,`tokio` and `tokio-async-await` as dependencies like so.
//!
//! ``` toml
//! tokio = { version = "0.1", features = ["async-await-preview"] }
//! tokio-async-await = "0.1"
//! futures-preview = { version = "0.3.0-alpha.13" }
//! ```
//!
//! Once, you have all these dependencies you can then use the attribute like so.
//!
//! ``` rust
//! #![feature(async_await, await_macro, futures_api)]
//!
//! extern crate futures;
//! extern crate tokio;
//! extern crate tokio_async_await;
//! extern crate tokio_async_await_test;
//!
//! use tokio_async_await_test::async_test;
//!
//! #[async_test]
//! async fn basic() {
//!     await!(example_async_fn());
//! }
//! ```
//!
//! This will spin up a tokio runtime and block on the `basic` function. This generally expands to look like this. Where `fut` is the test future you are running.
//!
//! ``` rust
//! #[test]
//! fn basic() {
//! 	// -- snip --
//!     let mut rt = Runtime::new().unwrap();
//!
//! 	rt.block_on(fut().unit_error().boxed().compat()).unwrap();
//! }
//! ```
//!
//! You can also use a current thread runtime by importing `use tokio_async_await_test::async_current_thread_test;`.

#![feature(async_await, await_macro, futures_api)]
#![recursion_limit = "128"]

extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate tokio_async_await;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

/// Run a future as a test, this expands to calling the `async fn` via `Runtime::block_on`.
#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use tokio::runtime::Runtime;
            use tokio_async_await::compat::backward;
            use futures::future::{FutureExt, TryFutureExt};

            let mut rt = Runtime::new().unwrap();

            #input

            rt.block_on(backward::Compat::new(#test_case_name().unit_error().boxed())).unwrap();
        }
    };

    TokenStream::from(expanded)
}

/// Run a future as a test, this expands to calling the `async fn` via `Runtime::block_on` with
/// the `current_thread::Runtime::block_on`.
#[proc_macro_attribute]
pub fn async_current_thread_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use tokio::runtime::current_thread::Runtime;
            use tokio_async_await::compat::backward;
            use futures::future::{FutureExt, TryFutureExt};

            let mut rt = Runtime::new().unwrap();

            #input

            rt.block_on(backward::Compat::new(#test_case_name().unit_error().boxed())).unwrap();
        }
    };

    TokenStream::from(expanded)
}
