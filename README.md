### Purpose

This repo contains a stress test of Warp's accepting new connections.

### How to run

Before running the server or the client you probably need to increase opened
files limit. On most Linux system `ulimit -n 1000000` should do the trick. You
may need to also increase the local port range:

```
sudo sysctl net.ipv4.ip_local_port_range="10000 61000"
```

In order to run the server:

```
cargo run --bin websockets-server --release
```

In order to run the client:

```
cargo run --bin stress-test --release
```

The client will open 50k new connections at roughly the same time.

The result for me looks something like:

```
connections: 0, elapsed: 0ms
connections: 15880, elapsed: 1009ms
connections: 18213, elapsed: 2026ms
connections: 20210, elapsed: 3052ms
connections: 22247, elapsed: 4065ms
connections: 24104, elapsed: 5071ms
connections: 25955, elapsed: 6074ms
connections: 27766, elapsed: 7097ms
connections: 29659, elapsed: 8100ms
connections: 31469, elapsed: 13690ms
connections: 50000, elapsed: 14691ms
```

In order to run a version that opens more than one port you can run:

```
PORTS=12 cargo run --bin websockets-server --release
```

and

```
PORTS=12 cargo run --bin stress-test --release
```

I used 12 ports as I have 12 CPU threads

Now the results look more like this for me:

```
connections: 0, elapsed: 0ms
connections: 44635, elapsed: 1000ms
connections: 50000, elapsed: 2001ms
```
