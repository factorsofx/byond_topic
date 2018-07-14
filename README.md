byondtopic
===========

Rust library for communicating with BYOND servers through `Topic()` calls.

How to Use
----------

```rust
use byondtopic::topic;

let topic_str = topic("goon1.goonhub.com:26100", "status");
```