# Lagom Projections

  [#2055](https://github.com/lagom/lagom/pull/2055)  introduces multiple changes in how Lagom handles projections. In this write-up we’re covering ubiquitous language, design decisions and questions and doubts about the current status.

### Ubiquitous Language

* *Projection* a process consuming an Akka Persistence Journal handling each event into a read table of a broker topic. Here, Projections only refer to `ReadSideProcessors`  and `TopicProducer`s (not Broker subscribers). This is different than the meaning of *projection* in `akka/akka-projection`.

* *ProjectionWorker* an actor where a `queryByTag` is run. This query starts off from an offset and consumes a single tag of the journal (aka a partition).

* *ClusterDistribution* an Akka extension introduced in Lagom to ensure all projections have a projection worker available for each partition at all times.

* *ProjectionRegistry* an internal registry tracking user requests for a projection or a worker to be stopped or started. The registry can also be queried to inspect what is the current requested Status and the current observed Status. Queries to the registry return the current local view of the *State*. The `ProjectionRegistry`is internal and only accesible via the `Projections` interface/trait.

* *State* the state of a projections registry is a collection of projections with extra data indicating the name of the projection, and details about its workers. Note that many projections may operate over the same journal. Each worker includes information about the particular tagname it is trackign. Note that multiple workers may track the same tagName because each worker is part of a different projection. Each worker also has a _key_ which is unique across the whole cluster. Finally, the data related to a worker that's part of the `State` includes a requested status and an observed status for the worker.

* *Status* `Stopped` or `Started`.

* *RequestedStatus* what the user demands. Even when the user wants a worker to be started, the worker may fail to start. 

* *ObservedStatus* the status the `ProjectionRegistry` thinks the worker is in. In nominal cases, this will eventually be equal to the `RequestedStatus`.  When they differ for a long period it could indicate issues on your deployment.

* *Default Requested Status* (aka `auto-start.enabled`) a user-provided config indicating what is the requested status when no user code has executed yet. Traditionally all projection workers in Lagom start eagerly but some customers and users want to disable a given worker or a full projection in case of head-of-line blocking or known third-party issues (imagine a scheduled Kafka downtime where I want my `TopicProducers` to be stopped).

* *Worker Coordinates* a tuple of a *projection name* and a *tag name*. Because a single Journal can be projected by many projections in parallel, the *tagName* is not a unique key but the tuple *(projectionName, tagName)* is.


### Design

#### `ProjectionRegistry`

The central piece of this feature is the Projection Registry which keeps some [`LWWMap`’s](https://doc.akka.io/docs/akka/current/distributed-data.html#maps) and a local-only map:

* each `ProjectionRegistry` instance keeps a list of `ActorRef` for the workers running on the same node. As the cluster topology changes and shards are reallocated, the local workers die/spawn and each intance of `ProjectionRegistry` across the cluster keeps their index updated. That list is kept as an index of the `ActorRef` and the *worker coordinates*.
 
* requested status LWWMap: anytime a request to change status comes in, it is first updated on the LWWMap with a `WriteMajority` ([maybe should migrate to `WriteTo`? ](https://github.com/lagom/lagom/issues/2130)). When a [`Changed`](https://doc.akka.io/docs/akka/current/distributed-data.html#subscribe) is observed for given *worker coordinates*, if those represent a worker that’s running locally then the requested status is propagated into the worker.
	
* observed status LWWMap:  similarly, each `ProjectionRegistry` watches the workers on the same node and maintains a `LWWMap` with the observed status. *Requested* and *Observed* may differ due to unreplicated data (in-flight), bugs, crash loops, etc…

It is important to note that each of the above can only be written by either the user code or the framework code:

* observed status is only written by framework code watching worker actors spawn and terminate

* actor index is only written by framework code observing actors spawn and terminate locally

* requested status is only written by user demand.

    * the requested status `LWWMap` is not even modified by the _default requested state_. The _default requested state_ is only a [fallback](https://github.com/lagom/lagom/blob/32b96cc20881e2b10d1ec7d554847db0a0ccdf64/projection/core/src/main/scala/com/lightbend/lagom/internal/projection/ProjectionRegistryActor.scala#L113) value so that *requested status* always indicates user runtime demand.

There is no active process trying to maintain the *observed status* equal to the *requested status* other than changes in the `LWWMap` backing the *requested state*.



#### `WorkerCoordinator` actor

Projections where traditionally implemented as a whole single class encapsulating the handling of `EnsureActive` (part of the [`ClusterDistribution` extension](https://github.com/lagom/lagom/blob/8269a747873eecedc7437253644304c48df497bb/cluster/core/src/main/scala/com/lightbend/lagom/internal/cluster/ClusterDistribution.scala#L82)) and their own messages and lifecycle management.

A `WorkerCoordinator` actor has been introduced and it holds all the interaction with `ClusterDistribution` and also the `ProjectionRegistry`. This refactor removes triplication code from multiple `ReadSideActor` and `TopicProducer` implementations but introducing some complexity in the `lagom/lagom` codebase. 

The WorkerCoordinator actor spawns as soon as it receives `EnsureActive(tagName)` messages from the cluster and remains alive forever. Even when the requested status for given `WorkerCoordinates` dictate the status to be `Stopped` the `WorkerCoordinator` will exist. When getting the first `EnsureActive` message, the `WorkerCoordinator` pings back to the `ProjectionRegistryActor` to indicate “I’m here, I represent a worker for this WorkerCoordinates”. That is, when the cluster starts up there are as many instances of `WorkerCoordinator` as (numProjections x numTags). The same is not true for the actual worker actors running the `queryByTag` streams. The actual worker actors are only started when requested and are stopped when requested.

An advantage of this pattern is that, because a `WorkerCoordinator` actor will know the `projectionName` and the `tagName` before the actual worker actor is created we can now make up a unique repeatable name for the worker actor. This unique `String` is called the `workerKey` and it helps detect duplicate actors in a given node (not across the cluster, though). It also helps in monitoring since the actor name now indicates exactly what projection and tag it is working on.
