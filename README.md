# PLC operation tools GUI

This is a personal project, a set of libraries and a simple egui tool for loading, modifying, and signing PLC
operations ([spec](https://web.plc.directory/spec/v0.1/did-plc)). I worked on this in order to learn more about
cryptographic signing keys (namely, secp256k1 and P-256), did:key, and especially did:plc.

<details>
<summary>DISCLAIMER:</summary>
This code was not originally meant to be made public in this state. A lot of documentation is missing, some features might be implemented only partially or not at all... you get the point.

All code is provided as-is, and I make no guarantees about its stability or how it runs on your machine. I tried to make
things clean, but they're only as clean as you'd expect for a personal project intended to explore a whole new topic.
</details>

---

# Quick start

If you just want to test out the GUI tool, you only need to clone the repo locally, make sure you have Rust installed (
`rustup`), build the `plc-interface` binary, and launch it. I'm on Rust `1.86.0-nightly` and use some experimental
features, but it should work well enough with the provided config files.

_Note that this has only been tested on my desktop, which runs Windows 11._

**Be aware that the gui application saves some state data**, it uses
the [default eframe settings](https://docs.rs/eframe/latest/eframe/fn.storage_dir.html) under the ID `PLC Interface` (
e.g. `AppData/Roaming/PLC Interface/*` for Windows). This location isn't cleared when you stop using the app, so if you
want to save 16kB of disk space, you know where to look.

**The application uses stdout** for errors and other info, including resulting signed PLC operations. The latter are
printed directly, and you should always see error messages, but you can enable extra details via the `RUST_LOG=info`
environment variable.

The app is also **completely offline** at runtime - everything is local, but you'll need to handle a few simple HTTP
requests on your own (see practical examples below).

## Practical examples

I wrote some detailed guides about how I'd go about using my PLC tool, you can follow these steps once you get the
visual app running. You'll also need **an HTTP client** _(such as Insomnia)_ to send some GET and POST requests - the
app is completely offline and makes no HTTP calls of its own! Besides that, I assume basic technical knowledge about
Bluesky (I mean, since you clicked on this repo, I'd assume you already know that did:plc is some sort of identifier),
but not much more besides that. If you managed to clone the repo, run `rustup`, and build the `plc-interface` binary,
you should be good.

The guides pretty much became blog posts, but I suppose it makes for a nicer read too:

- [Adding your first rotation key](./guides/add_rot_key.md)
- [Signing your first PLC operation](./guides/self_signing.md) _(WIP)_

---

# GUI

The interface consists of two main sections: The key store, and the PLC operation editor interface itself.

## Key store

The key store simply loads and keeps track of PEM-encoded signing keys located in a directory (kinda like ssh keys).
This tries to default to `.key_store` relative to the working dir.

- Click 🔁 to **reload** all keys
- Expand the dropdown and **generate** a new random key (which then gets saved to the same location with its did:key
  representation as its name)

## PLC operation editor

The PLC operation editor interface mirrors the JSON structure of a PLC operation object:

- **Also known as:** a new-line-separated list of `at://` aliases
- **Rotation keys:** an ordered array of did:keys.
    - Red-colored keys have an invalid format, green-colored keys are keys you own (i.e. keys that were found in your
      key store).
- **Verification methods:** key-value map of services and did:keys _(you'll most likely care only about `atproto`)_
- **Services:** key-value map of services and endpoints _(again, you'll most likely only care about `atproto_pds`)_.
    - _Does not support adding/removing entries in the GUI, but you can still edit your endpoints._
- **Previous CID:** the `prev` field of the operation. This can be calculated from another signed operation, or
  cleared (for a genesis operation).

You may load a signed PLC operation with a button at the top, which also generates a new `prev` CID referencing that
operation (in other words, this does NOT copy the original CID). The *Previous CID* section allows you to generate and
replace _only_ the CID from a signed operation.

Finally, once you're done modifying the PLC operation, you may either print the whole unsigned operation (no `sig`
field) as JSON, or generate a signature using your selected (and owned!) rotation key (selected using a radio button to
the left of each rotation key).

# Libraries

Besides the main binary, the codebase also contains several libraries. Importantly, there's **a custom implementation of
did:key** and **did:plc**, as well as some helper structs and traits e.g. for the current "blessed" signing methods.

_I decided to implement both did:key and did:plc on my own, because the existing libraries I found didn't work well for
my purposes, and I ran into issues with dependencies. Besides, it was a good opportunity to properly learn more about
everything myself._

As stated before, the code is missing a lot of documentation, though you'll at least find some unit tests and some
example uses in `did-plc/src/main.rs`. The rest of the examples is effectively the whole `plc-interface` crate.