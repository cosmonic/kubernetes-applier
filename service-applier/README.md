# Kubernetes Service Applier Actor

This actor is for use in Kubernetes <-> wasmCloud compatibility. It works by listening on the
wasmCloud event messaging topic for new linkdefs between actors and `wasmcloud:httpserver`
contracts. When one of these appear a service will be created to point at the proper port exposed on
the httpserver. When the link is deleted, the service will be removed.

This is also meant to be used as a template for an actor when using `wash new actor` for those who
wish to customize what resources are created on link definitions.

## How to use

In order to use this actor, you'll need the NATS `wasmcloud:messaging` provider and the [Kubernetes
Applier Provider](../applier):

```console
$ wash ctl start provider wasmcloud.azurecr.io/applier:0.2.0
$ wash ctl start provider wasmcloud.azurecr.io/nats_messaging:0.11.5
```

Once you have started the providers, you can start the actor:

```console
$ wash ctl start actor wasmcloud.azurecr.io/service_applier:0.2.0
```

Then you'll need to link the providers to the actor. For instructions on how you can configure the
applier provider, see [its README](../applier/README.md). For the NATS provider, you will need to
connect to the same NATS cluster that your wasmcloud hosts are connected to. This can be specified
with the `URI` parameter (e.g. `URI=nats://localhost:4222`). You'll also need to set which
subscription to listen to: `SUBSCRIPTION=wasmbus.evt.<lattice-prefix>`, where lattice prefix is the
same prefix you specified for your hosts, by default, this is `default` (so your configuration would
be `SUBSCRIPTION=wasmbus.evt.default`);

NOTE: All `Service`s will be created will be in the default namespace of the kubeconfig you use for
the link definition between this actor and the applier provider. However, this is often desired
behavior as you can run this actor on a host inside of Kubernetes, which means you can use service
account credentials. By default, these credentials are scoped to the namespace where the hosts are
running, which is where the `Service` should be at anyway

### Requirements for Hosts running in Kubernetes

If you'd like your existing applications running in Kubernetes to be able to connect to applications
running in wasmCloud, we recommend creating a "routing tier" of wasmCloud hosts. This means you will
have one `Deployment` of pods running wasmCloud hosts that are just for running actors and other
providers. You will then have a second `Deployment` of pods running wasmCloud hosts that all have
the HTTP server provider running on them. Each of these pods should have the label and value
`wasmcloud.dev/route-to=true` on them in order to have traffic routed to them. Essentially, the
`Service`s created by this actor direct traffic to those HTTP servers, all of which will have the
port you configured in your link definition available. Once the traffic has hit those HTTP servers,
it will be transmitted to actors running in the lattice, whether those are running inside or outside
of Kubernetes. A simple diagram is below:

```
┌──────────────────────────────────┐ ┌─────────┐
│            Kubernetes            │ │         │ Other
│                                  │ │         │ ┌────┐
│            ┌────────┐            │ │         │ │    │
│            │Service │            │ │         ├─►    │
│            │        │            │ │         │ └────┘
│     ┌──────┴┬───────┼───────┐    │ │         │
│     │       │       │       │    │ │         │ ┌────┐
│  ┌──▼─┐  ┌──▼─┐  ┌──▼─┐  ┌──▼─┐  │ │         │ │    │
│  │    │  │    │  │    │  │    │  │ │         ├─►    │
│  │    │  │    │  │    │  │    │  │ │         │ └────┘
│  └─┬──┘  └────┘  └────┤  └──┬─┘  │ │         │
│    │     Router Hosts │     │    │ │         │ ┌────┐
│    │       │          └─────┴────┼─► Lattice │ │    │
│    │       │                     │ │         ├─►    │
│    └──────►└─────────────────────► │         │ └────┘
│                                  │ │         │
│  ┌────┐  ┌────┐  ┌────┐  ┌────┐  │ │         │ ┌────┐
│  │    │  │    │  │    │  │    │  │ │         │ │    │
│  │    │  │    │  │    │  │    ◄──┼─┤         ├─►    │
│  └────┘  └────┘  └────┘  └────┘  │ │         │ └────┘
│          Normal Hosts            │ │         │
│                                  │ │         │ ┌────┐
│                                  │ │         │ │    │
│                                  │ │         ├─►    │
│                                  │ │         │ └────┘
└──────────────────────────────────┘ └─────────┘ Hosts
```

## See it in action

The easiest way to see this in action is to start the httpserver provider
(wasmcloud.azurecr.io/httpserver:0.14.6) and the echo actor (wasmcloud.azurecr.io/echo:0.3.2) and
then link them (you can do this in washboard as well if you prefer a GUI):

```console
$ wash ctl link put MBCFOPM6JW2APJLXJD3Z5O4CN7CPYJ2B4FTKLJUR5YR5MITIU7HD3WD5 VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver 'address=0.0.0.0:8081'
⡃⠀ Defining link between MBCFOPM6JW2APJLXJD3Z5O4CN7CPYJ2B4FTKLJUR5YR5MITIU7HD3WD5 and VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M ... 
Published link (MBCFOPM6JW2APJLXJD3Z5O4CN7CPYJ2B4FTKLJUR5YR5MITIU7HD3WD5) <-> (VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M) successfully
```

Then you can see that the service was created by running:

```console
$ kubectl get svc
NAME                                                       TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)                                                 AGE
kubernetes                                                 ClusterIP   10.96.0.1       <none>        443/TCP                                                 4d1h
mbcfopm6jw2apjlxjd3z5o4cn7cpyj2b4ftkljur5yr5mitiu7hd3wd5   ClusterIP   10.96.170.75    <none>        8081/TCP                                                10s
```

The service name is the lowercased actor ID, so you can easily identify which actor it is pointing
at.
