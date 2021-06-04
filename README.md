# Exchange

## High-level Architecture

The project consists of two crates:

- `exchange`: The library that encapsulates all the business logic for the payment platform
- `exchange-cli`: This is just a wrapper around the library. This way, the library can be independently used in a server environment as well.

## Design Decisions

- In the exchange crate, unsuccessful transactions are hard errors. In the CLI, these errors will merely be logged but otherwise ignored (as stated in the task description). This allows for more granular handling of specific error cases in the future.
- CSV lines are read one-by-one instead of loading the entire dataset upfront?
- Test-driven development: All code should have unit tests. I've also added a few integration tests (see `fixtures` folder)
- No unsafe code.
- Two separate crates to not pollute the namespace and keep dependencies separate.
- Locked accounts can no longer be modified once locked

## Module documentation

Each module is documented using doc-comments. You can open them with

```
cargo doc --open --workspace
```

Below are some additional design decisions I've made.

### Handling Monetary Values

Handling monetary values is difficult and there are many caveats.

There are quite a few options how to do this

- parse::<f64>
- split at dot
- letting an external money crate handle it
- ...

Each have their pros and cons and there are a lot of edge-cases to consider.
In the end I was not sure about which
formats the automated tests would accept, e.g. is `1.` a correct input? The spec
doesn't mention that case (perhaps on purpose). In the end I decided to use
an external crate (rust_decimal) for that, as I liked the source code and it was easy to integrate with serde.

**In a corporate environment, this would be aligned with the team**

### Exchange

The exchange is the single source of truth.
It only stores valid transactions.

Before getting committed, every transaction is validated and bad ones get
rejected. The exchange needs to be aware of the valid clients in the system.
As such, there is not other place in the system where the validity of a
transaction can be ensured (I guess).

## Testing Methodology

The type system helps in allowing only syntactically valid transactions by
using an enum for all allowed types.
It's neat because no case can be forgotten when matching against it.
I tried to make invalid state unrepresentable. For example, if a client gets
locked, it can not be modified anymore, which gets enforced by the typesystem.

To run the unit-tests, call

```
cargo test
```

## Future work

There are some things I would improve in a future version:

- Add fuzzing to find edge-cases, which are not covered yet.
- Use a lockless datastructure for transactions for higher throughput.

## Feedback

While working on the case-study, I had a few ideas on how to make it
a little more convenient for future candidates:

- In the table for the output column you mention a `locked` field, which has the
  following description: "An account is locked if a charge back occurs". Further
  down in the `Chargeback` section of the document, you call this state
  `frozen`. This was initially confusing me as I wasn't sure if that's the same
  field. Perhaps you want to rename that to `locked` as well.
- The document states: "If a client doesn't exist create a new record". I was
  not sure if that would also be true even if the transaction was NOT valid. If
  that's the case, a malicious user could potentially spam the exchange by
  creating a lot of fake transactions (which all get rejected) and register fake
  clients. I'm just going to assume that you'll expect all given client ids in
  the output, so I'll create them in all cases.
- In the second sample output the last field name is missing. It should probably
  be

  ```csv
  client,available,held,total,locked
  ```

  Perhaps that also was on purpose, though.

## Additional Crates Used

- [anyhow]: Amazing lib for ad-hoc CLI errors
- [thiserror]: Great for library errors to match on
- [rust_decimal]: Because it integrates well with serde
- [clippy](https://github.com/rust-lang/rust-clippy) (Not strictly a library, but I'm just a huge fan of static analysis. Even run [my own platform](analysis-tools) for that, heh)

[anyhow]: https://github.com/dtolnay/anyhow
[thiserror]: https://github.com/dtolnay/thiserror
[rust_decimal]: https://docs.rs/rust_decimal/
[analysis-tools]: https://analysis-tools.dev/
