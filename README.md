# p2p-exec

Simple peer-to-peer execution of basic math operations. Built on [libp2p](https://libp2p.io/).

## Usage

1. Open two terminals
2. run `cargo run` in each to start a node

Each node should output something like the following, indicating the peers discovered each other.

```

Local peer id: PeerId("12D3KooWA7LShSoWyTEKW6qZtLCrxeqzqP3mNfFwaaaW4uJ2gLft")
listener id: ListenerId(1)
Listening on "/ip4/127.0.0.1/tcp/45397"
Listening on "/ip4/192.168.2.131/tcp/45397"
Listening on "/ip4/172.20.0.1/tcp/45397"
Listening on "/ip4/172.17.0.1/tcp/45397"
Listening on "/ip4/172.18.0.1/tcp/45397"
discovered peer: PeerId("12D3KooWRvvA82V3BJrMttRyzcp4dYcv6SqMZdo64peghCh1DnuT")
discovered peer: PeerId("12D3KooWRvvA82V3BJrMttRyzcp4dYcv6SqMZdo64peghCh1DnuT")
discovered peer: PeerId("12D3KooWRvvA82V3BJrMttRyzcp4dYcv6SqMZdo64peghCh1DnuT")
discovered peer: PeerId("12D3KooWRvvA82V3BJrMttRyzcp4dYcv6SqMZdo64peghCh1DnuT")
discovered peer: PeerId("12D3KooWRvvA82V3BJrMttRyzcp4dYcv6SqMZdo64peghCh1DnuT")
discovered peer: PeerId("12D3KooWH2ozJPojBR26uAeXXB8rHRsEXX6Lj8aB2sva2ksJKHn7")
discovered peer: PeerId("12D3KooWH2ozJPojBR26uAeXXB8rHRsEXX6Lj8aB2sva2ksJKHn7")
discovered peer: PeerId("12D3KooWH2ozJPojBR26uAeXXB8rHRsEXX6Lj8aB2sva2ksJKHn7")
discovered peer: PeerId("12D3KooWH2ozJPojBR26uAeXXB8rHRsEXX6Lj8aB2sva2ksJKHn7")
discovered peer: PeerId("12D3KooWH2ozJPojBR26uAeXXB8rHRsEXX6Lj8aB2sva2ksJKHn7")
```

In either terminal, enter a basic math operation such as:

```
add 5 6
sub 623 23
mult 42 313
div 100 20
```

After hitting enter, you should see the operation be executed by the second terminal:

```
Request(ExecRequest { function: Add, args: (27, 12093) }) from PeerId("12D3KooWAuyzhL2ei6AuEuedLcVeMcEkbCdknEstHVvHtPEB98UR")
```

And the result be returned to sending peer in the first terminal

```
add 27 12093
publishing op: Request(ExecRequest { function: Add, args: (27, 12093) })
Response(ExecResponse { result: 12120 }) from PeerId("12D3KooWMWDcvVA8jhn3ggHZr6XmvUs73FGjp3AjRek98w8HjA2k")
received result: 12120
```

You may start additional peers, but you will see duplicate responses from each peer. As well as peers receiving the result that shouldn't.

### TODO

- better msg subscribing, set recipient in result
- allow peers to register which operations they can perform
- get relay server working, to use [hole punching](https://docs.rs/libp2p/latest/libp2p/tutorials/hole_punching/index.html)
