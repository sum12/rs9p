9P
==


This is an implementation of the Plan 9 9p remote filesystem protocol.
The implementations goal to play with rust language and understand the protocol too.
Because of that you dont want to use this for anything serious. 


Some of the features include
- Simplistic model
- Bulitin Auth model
- file permission control 
- [Builtin linux server support](https://github.com/torvalds/linux/blob/master/Documentation/filesystems/9p.rst)


## Docs
There is no documentation at all. but the code in [proto](./proto) should readable and [client](./client) is still evolving

The implementation is not complete. There are 3 crates

- [proto](./proto)
    This is the colection of building boxes which are used by the client and server to facilitate commnication

- [client](./client)
    It contains a client which can be used to talk to any server. Look at [main.rs](./src/main.rs) for a sample usage

- server (to be continued....)



LISCENCE:

GPL
