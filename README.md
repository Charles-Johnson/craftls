<p align="center">
  <img width="460" src="https://github.com/3andne/craftls/assets/52860475/26574ee5-abf3-4eca-98ac-130fef0e79eb">
</p>

<p align="center">
Craftls is a fork of the Rustls library with customizable ClientHello fingerprint.
</p>

# Status

Craftls is under active development. We aim to maintain
reasonable API surface stability but the API may evolve as we make changes to accommodate
new features or performance improvements.

## Changelog

The detailed list of changes in each release can be found at
https://github.com/3andne/craftls/releases.

# Documentation

https://docs.rs/craftls/

# Approach

`Craftls` is a TLS library that aims to be a drop-in replacement of `Rustls`, offering customizable `ClientHello` while maintaining robust security and ease of use.

## Current functionality (with default crate features)

* Capabilities inherited from [Rustls](https://github.com/rustls/rustls?tab=readme-ov-file#current-functionality-with-default-crate-features)
* Customization options for `ClientHello` extensions
* Customization options for `ClientHello` cipher suites.
* Support for client-side Certificate Compression using `zlib`, `zstd`, and `brotli` compression methods ([rfc8879](https://datatracker.ietf.org/doc/html/rfc8879)).
* ClientHello padding extension ([rfc7685](https://datatracker.ietf.org/doc/html/rfc7685)).
* Grease extension ([rfc8701](https://datatracker.ietf.org/doc/html/rfc8701))
* TLS ClientHello extension permutation ([chrome](https://chromestatus.com/feature/5124606246518784))
* Predefined browser fingerprints
  * `CHROME_108`
  * `CHROME_112`
  * `SAFARI_17_1`
  * `FIREFOX_105`

## Non-features

We will not be supporting any non-features listed in [Rustls README](https://github.com/rustls/rustls?tab=readme-ov-file#non-features), including deprecated TLS versions and outdated cipher suites. 

While these non-features may be included in browser fingerprints for completeness, any server attempt to use them will result in the termination of the connection. Most modern and secure servers do not utilize these outdated options, so this measure should not impact regular use.
For reasons [explained in the manual](https://docs.rs/rustls/latest/rustls/manual/_02_tls_vulnerabilities/index.html),
rustls does not and will not support:

* SSL1, SSL2, SSL3, TLS1 or TLS1.1.
* RC4.
* DES or triple DES.
* EXPORT ciphersuites.
* MAC-then-encrypt ciphersuites.
* Ciphersuites without forward secrecy.
* Renegotiation.
* Kerberos.
* TLS 1.2 protocol compression.
* Discrete-log Diffie-Hellman.
* Automatic protocol version downgrade.
* Using CA certificates directly to authenticate a server/client (often called "self-signed
certificates"). _Rustls' default certificate verifier does not support using a trust anchor as
both a CA certificate and an end-entity certificate in order to limit complexity and risk in
path building. While dangerous, all authentication can be turned off if required --
see the [example code](https://github.com/rustls/rustls/blob/992e2364a006b2e84a8cf6a7c3eaf0bdb773c9de/examples/src/bin/tlsclient-mio.rs#L318)_.

There are plenty of other libraries that provide these features should you
need them.

### Platform support

While Rustls itself is platform independent, by default it uses
[`ring`](https://crates.io/crates/ring) for implementing the cryptography in
TLS. As a result, rustls only runs on platforms
supported by `ring`. At the time of writing, this means 32-bit ARM, Aarch64 (64-bit ARM),
x86, x86-64, LoongArch64, 32-bit & 64-bit Little Endian MIPS, 32-bit PowerPC (Big Endian),
64-bit PowerPC (Big and Little Endian), 64-bit RISC-V, and s390x. We do not presently
support WebAssembly.
For more information, see [the supported `ring` target platforms][ring-target-platforms].

By providing a custom instance of the [`crypto::CryptoProvider`] struct, you
can replace all cryptography dependencies of rustls.  This is a route to being portable
to a wider set of architectures and environments, or compliance requirements.  See the
[`crypto::CryptoProvider`] documentation for more details.

Specifying `default-features = false` when depending on rustls will remove the
dependency on *ring*.

Rustls requires Rust 1.61 or later.

[ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64
[crypto::CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/trait.CryptoProvider.html

# Example code

See `examples/src/bin/craftclient.rs`

## Configuration

### Direct Usage

To use `craftls` directly, just add `craftls` in your `Cargo.toml`.

### As a `rustls` Replacement

If you wish to replace `rustls` with `craftls` in nested dependencies (dependencies of dependencies), you can use the [patch.crates-io] section in your Cargo.toml:

```toml
[patch.crates-io]
rustls = { git = 'https://github.com/3andne/craftls.git', tag = "your version" }
```

Make sure to substitute "your version" with the specific version tag of craftls you intend to use. **This patch will ensure that `craftls` is used in place of `rustls` throughout your project, including within libraries like `tokio-rustls`**.

## Usage

`Craftls` is designed to be a drop-in replacement for `Rustls` with an additional feature for specifying TLS fingerprints. Below is a guide on how to configure the `ClientConfig` in `Craftls` to use a specific fingerprint.

```rust
let mut config: rustls::ClientConfig = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth()
    .with_fingerprint( // Specifies the fingerprint we want to use, i.e., CHROME v108
        rustls::craft::CHROME_108
            .builder(),
    );
```

After setting up the ClientConfig with the preferred fingerprint, you can proceed as you would with Rustls. The rest of the API remains consistent with the Rustls library.

### Use with http clients

Http clients such as `hyper` internally manage ALPN settings. They may raise issues if ALPN is set externally. Use the following configuration to avoid the panic:

```rust
let mut config: rustls::ClientConfig = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth()
    .with_fingerprint(
        rustls::craft::CHROME_108
            .builder()
            .do_not_override_alpn(), // let the http client manage the alpn
    );
```

### Use with http/1.1 or non-http clients

**Warning**: browsers are `h2` clients. `Http1.1` and non-http variations deviate from browsers standard browser behaviors and should be used carefully.

```rust
let mut config: rustls::ClientConfig = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth()
    .with_fingerprint(
        rustls::craft::CHROME_108
            .test_alpn_http1 // alpn: ["http/1.1"]
            .builder(),
    );
```

Or

```rust
let mut config: rustls::ClientConfig = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth()
    .with_fingerprint(
        rustls::craft::CHROME_108
            .test_no_alpn // no alpn extension
            .builder(),
    );
```

# License

Craftls is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.

# Code of conduct

This project adopts the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
Please email rustls-mod@googlegroups.com to report any instance of misconduct, or if you
have any comments or questions on the Code of Conduct.

---

Icons by [icons8](https://icons8.com/)