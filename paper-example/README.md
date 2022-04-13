# Paper Example

Example to show the easy transition to distributed applications with Actix-Telepathy

## Usage

### Help
```shell
cargo r -- --help

> paper-example 
> Example to show the easy transition to distributed applications with Actix-Telepathy
> 
> USAGE:
>     paper-example [OPTIONS] --own-addr <OWN_ADDR>
>
> OPTIONS:
>     -h, --help                       Print help information
>         --own-addr <OWN_ADDR>        Own public IP address to listen to
>         --seed-nodes <SEED_NODES>    Addresses of already known cluster nodes
```


### Run
This example can be run on one computer, that will act as 2 cluster nodes.

#### Cluster node 1
```shell
# cargo r -- --own-addr <ip-addr> [--seed-nodes <ip-addr1> [--seed-nodes <ip-addr2> [...]]]
cargo r -- --own-addr 127.0.0.1:1992 --seed-nodes 127.0.0.1:1993

> MyMessage received
```

#### Cluster node 2 (in different terminal window)
```shell
cargo r -- --own-addr 127.0.0.1:1993

> MyMessage received
```
