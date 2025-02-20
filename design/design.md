Zebra Design Document
=====================

This document sketches the future design for Zebra.

Desiderata
==========

The following are general desiderata for Zebra:

* [George's list..]

* As much as reasonably possible, it and its dependencies should be
  implemented in Rust.  While it may not make sense to require this in
  every case (for instance, it probably doesn't make sense to rewrite
  libsecp256k1 in Rust, instead of using the same upstream library as
  Bitcoin), we should generally aim for it.

* As much as reasonably possible, Zebra should minimize trust in
  required dependencies.  Note that "minimize number of dependencies"
  is usually a proxy for this desideratum, but is not exactly the same:
  for instance, a collection of crates like the tokio crates are all
  developed together and have one trust boundary.
  
* Zebra should be well-factored internally into a collection of
  component libraries which can be used by other applications to
  perform Zcash-related tasks.  Implementation details of each
  component should not leak into all other components.
  
* Zebra should checkpoint on Sapling activation and drop all
  Sprout-related functionality not required post-Sapling.
  
Internal Structure
==================

The following is a list of internal component libraries (crates), and
a description of functional responsibility.

`zebra-chain`
--------------

### Internal Dependencies

None: this the core data structure definitions.

### Responsible for

- definitions of commonly used data structures, e.g.,
  - `Block`,
  - `Transaction`,
  - `Address`,
  - `KeyPair`...

- definitions of core traits, e.g.,
  - `ZcashSerialize` and `ZcashDeserialize`, which perform
    consensus-critical serialization logic.

- context-free validation behaviour, e.g., signature, proof verification, etc.

### Exported types

- [...]

`zebra-network`
----------------

### Internal Dependencies

- `zebra-chain`

### Responsible for

- definition of a sane, internal request/response protocol
- provides an abstraction for "this node" and "the network" using the
  internal protocol
- dynamic, backpressure-driven peer set management
- per-peer state machine that translates the internal protocol to the
  Bitcoin/Zcash protocol
- tokio codec for Bitcoin/Zcash message encoding.

### Exported types

- `Request`, an enum representing all possible requests in the internal protocol;
- `Response`, an enum representing all possible responses in the internal protocol;
- `AddressBook`, a data structure for storing peer addresses;
- `Config`, a configuration object for all networking-related parameters;
- `init<S: Service>(Config, S) -> (impl Service,
  Arc<Mutex<AddressBook>>)`, the main entry-point.

The `init` entrypoint constructs a dynamically-sized pool of peers
sending inbound requests to the provided `S: tower::Service`
representing "this node", and returns a `Service` that can be used to
send requests to "the network", together with an `AddressBook` updated
with liveness information from the peer pool.  The `AddressBook` can
be used to respond to inbound requests for peers.

All peerset management (finding new peers, creating new outbound
connections, etc) is completely encapsulated, as is responsibility for
routing outbound requests to appropriate peers.

`zebra-storage`
----------------

### Internal Dependencies

- `zebra-chain` for data structure definitions.

### Responsible for

- block storage API
  - operates on raw bytes for blocks
  - primarily aimed at network replication, not at processing
  - can be used to rebuild the database below
- maintaining a database of tx, address, etc data
  - this database can be blown away and rebuilt from the blocks, which
    are otherwise unused.
  - threadsafe, typed lookup API that completely encapsulates the
    database logic
  - handles stuff like "transactions are reference counted by outputs"
    etc.
- providing `tower::Service` interfaces for all of the above to
  support backpressure.

### Exported types

- [...]

`zebra-script`
---------------

### Internal Dependencies

- ??? depends on how it's implemented internally

### Responsible for

- the minimal Bitcoin script implementation required for Zcash

### Notes

This can wrap an existing script implementation at the beginning.

If this existed in a "good" way, we could use it to implement tooling
for Zcash script inspection, debugging, etc.

### Questions

- How does this interact with NU4 script changes?

### Exported types

- [...]

`zebra-consensus`
------------------

### Internal Dependencies

- `zebra-chain`
- `zebra-storage`
- `zebra-script`

### Responsible for

- consensus-specific parameters (network magics, genesis block, pow
  parameters, etc) that determine the network consensus
- consensus logic to decide which block is the current block
- all context-dependent validation logic, e.g., determining whether a
  transaction is accepted in a particular chain state context.

### Exported types

- [...]

`zebra-rpc`
------------

### Internal Dependencies

- `zebra-chain` for data structure definitions
- `zebra-network` possibly? for definitions of network messages?

### Responsible for

- rpc interface

### Exported types

- [...]

`zebra-client`
-----------------

### Internal Dependencies

- `zebra-chain` for structure definitions
- `zebra-storage` for transaction queries and client/wallet state storage
- `zebra-script` possibly? for constructing transactions

### Responsible for

- implementation of some event a user might trigger
- would be used to implement a wallet
- create transactions, monitors shielded wallet state, etc.

### Notes 

Communication between the client code and the rest of the node should be done
by a tower service interface. Since the `Service` trait can abstract from a
function call to RPC, this means that it will be possible for us to isolate
all client code to a subprocess.

### Exported types

- [...]

`zebrad`
---------

Abscissa-based application which loads configs, all application components,
and connects them to each other.

### Responsible for

- actually running the server
- connecting functionality in dependencies

### Internal Dependencies

- `zebra-chain`
- `zebra-network`
- `zebra-storage`
- `zebra-consensus`
- `zebra-client`
- `zebra-rpc`

Unassigned functionality
------------------------

Responsibility for this functionality needs to be assigned to one of
the modules above (subject to discussion):

- [ ... add to this list ... ]
