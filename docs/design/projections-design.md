# Lagom Projections

 [#2055](https://github.com/lagom/lagom/pull/2055) introduces multiple changes in how Lagom handles projections. In this write-up we’re covering ubiquitous language, design decisions and questions and doubts about the current status.

### Ubiquitous Language

*Projection* a process consuming an Akka Persistence Journal handling each event into a read table of a broker topic. Here, Projections only refer to `ReadSideProcessors`  and `TopicProducer`s (not Broker subscribers. This is different than the meaning of *projection* in `akka/akka-projection`
*ProjectionWorker* an actor where a `queryByTag`is run. This query starts off from an offset and consumes a single tag of the journal (aka a partition).
*ClusterDistribution* an Akka extension introduced in Lagom to ensure all projections have a projection worker available for each partition at all times.
*ProjectionRegistry* a registry where users may request a projection or a worker to stop or start. The registry can also be queried to inspect what is the current requested Status and the current observed Status. Queries to the registry return the current local view of the *State*
*State* the projections state is a list of projections. Each projection is a name and a list of workers. Each worker has a _name_(equivalent to the tagName), a _key_ (unique across the whole cluster), a requested status and an observed status.
*Status* Stopped or Started
*RequestedStatus* what the user demands. Even when the user wants a worker to be started, the worker may fail to start. 
*ObservedStatus* the status the ProjectionRegistry thinks the worker is in. 
*Default Initial State* ([unimplemented](https://github.com/lagom/lagom/issues/2120)) a user-provided config indicating what is the requested status when no user code has executed yet. Traditionally all projection workers in Lagom start eagerly but some customers and users want to disable a given worker or a full projection in case of head-of-line blocking or known third-party issues (imagine a scheduled Kafka downtime where I want my `TopicProducers` to be stopped).
*WorkerCoordinates* a tuple of a *projection name* and a *tag name* . Because a single Journal can be projected by many projections in parallel, the *tagName* is not a unique key but the tuple *(projectionName, tagName)* is.

### Design

#### `ProjectionRegistry`

The central piece of this feature is the Projection Registry which keeps some `LWWMap`’s and a local-only map:
	 * each `ProjectionRegistry` instance keeps a list of `ActorRef`  for the workers running on the same node. As the cluster topology changes and shards are reallocated, the workers die/spawn and each `ProjectionRegistry` keeps their index updated. That list is kept as an index of the `ActorRef` and the *worker key*.
	 * requested status LWWMap: anytime a request to change status comes in, it is first updated on the LWWMap with a `WriteMajority` ([maybe should migrate to `WriteTo`? ](https://github.com/lagom/lagom/issues/2130)) ¿*@akka-team*?. When a `Changed` is observed for a given *worker key*, if it represents a worker that’s running locally then the requested status is propagated into the worker. (¿[What happens on _upscale_](https://github.com/lagom/lagom/issues/2125)? ¿*@akka-team*?)
	 * observed status LWWMap:  similarly, each `ProjectionRegistry` watches the workers on the same node and maintains a `LWWMap` with the observed status. *Requested* and *Observed* may differ due to bugs, crash loops, etc…
It is important to note that each of the above can only be written by either the user code or the framework code:
	 * observed status is only written by framework code watching worker actors spawn and terminate
	 * actor index is only written by framework code observing actors spawn and terminate locally
	 * requested status is only written by user demand.
		 * the requested status LWWMap is not even modified by the _default initial state_. The _default initial state_ is only a [fallback](https://github.com/lagom/lagom/blob/6887d01df568c6ca29c74d10c35d6c3c32c7183f/projection/core/src/main/scala/com/lightbend/lagom/internal/projection/ProjectionRegistryActor.scala#L103) value so that *requested status* always indicated runtime demand.
 There is no active process trying to maintain the *observed state* equal to the *requested state* other than changes in the LWWMap backing the *requested state*

#### `WorkerHolderActor` (name is [WIP](https://github.com/lagom/lagom/issues/2126))

Projections where traditionally implemented as a whole single class encapsulating the handling of `EnsureActive` (part of the `ClusterDistribution` extension) and their own messages and lifecycle management.

A `WorkerHolderActor` has been introduced and it holds all the interaction with `ClusterDistribution` and also the `ProjectionRegistry`removing triplication code from `ReadSideActor` and `TopicProducer` implementations but introducing some complexity. 

The `WorkerHolderActor` is always alive and receives `EnsureActive(tagName)` messages from the cluster. When getting the first of those messages, it pings back to the `ProjectionRegistry` to indicate “I’m here, I’m a worker with this *WorkerCoordinates*”.  Even when the requested status for given *WorkerCorrdinates* dictate the status to be *Stopped* the `WorkerHolderActor` will exist. That is, when the cluster starts up there are are many instances of `WorkerHolderActor` as *projections * tags*. The same is not true for the actual worker actors holding the `queryByTag` streams. The actual workers are only started when requested and are stopped when requested.

An advantage of this pattern is that, because a `WorkerHolderActor` will know the `projectionName` and the `tagName` before the actual worker actor is created we can now make up a unique repeatable name for the worker actor. This unique `String` is called the `workerKey` and it helps detect duplicate actors in a given node (not across the cluster, though). It also helps in monitoring since the actor name now indicates exactly what projection and tag it is working on.