# NetVisor

NetVisor is a host network topology visualization tool. It provides a command
line tool that generates graphic images containing diagrams of the current host
network configuration.

## Why?

In complex systems where there are multiple interfaces configured across
multiple network namespaces it is hard to identify if the interfaces are
configured correctly. NetVisor try to solve that problem by providing a visual
representation of the network topology.

## Scope

The project is aim to be used on GNU/Linux systems and currently the idea of
supporting other operative systems is out of the table. Having said this, the
situation could change in the future.

## Technology

### Language

Rust - Mainly because I like it and fits the purpose of the project. Other
languages like C/C++ were considered aswell but I prefer to take advantage from
the Rust's memory safety features. In addition, the Rust community is awesome,
that is another important thing that was considered.
