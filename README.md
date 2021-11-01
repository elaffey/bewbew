# bewbew

## Toy Rust web app

## cli
A simple cli application to set up secrets and key pairs. Two subcommands are supported. 
1. `gen_salt_secret` makes a binary file with 16 bits of randomness
2. `gen_key_pair` makes a Ed25519 key pair

## sdk
This is the central business logic of the application. This library does not manage state directly. Rather, all handles that manage stateful activities are wrapped in the `State` struct. 

Further, this component does not concern itself with any wire protocol. This component exports public functions in the `apis` module.

## store
This component manages any state that needs to be persisted. Internally the Rust embedded database [sled](https://github.com/spacejam/sled) is used. The intention is that the specifics of how this works is abstracted. 

## types
Types that are used by public APIs in `store` and `sdk` are exported here. [bincode](https://github.com/bincode-org/bincode) is used as the serialization format for the database and the HTTP APIs. Doing so limits clients to use Rust and the same version of `bincode` but for a toy application sharing types across the client and server works out nicely. 

## server
This is a [hyper](https://github.com/hyperium/hyper) HTTP server. It loads and manages the `sdk/State` struct. It then routes incoming requests to the corresponding `sdk/apis` API. The idea is that this contains little logic, mostly plumbing to get the sdk exposed over HTTP.

## client
This uses [reqwest](https://github.com/seanmonstar/reqwest) to call the server. The idea is that there is a little plumbing to manage all the HTTP stuff and to create an API that corresponds with the functions exported by `sdk/store`. This should make the client easy to work with.

---

There are a few benefits to this architecture. Firstly, keeping all the logic purely functional keeps it easy to test and compose. Keeping all the stateful stuff neatly separated makes it easy to manage, swap out of alternative implementations, and mock. 

Also possible is embedding the `sdk` in the client directly. Using this architecture, it's possible for the front end dev experience to be totally local within a single executable. 