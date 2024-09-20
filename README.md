# EventRC

## SubProjects

### Director
 - Injests Events, validates them, and sends to Producer
 - Registers Actors and subscribes them to event types
 - Informs Actors of new events recieved

### Producer
  - Recieves events from directory and appends to event store
  - Produces EventStreams for Actors to consume (?)
  - Produces Messages/Sockets for internal actions to be taken.

### Actors
  - Registers self with Director for subscribing to certain events
  - Performs actions based on events occuring such as creating or updating
    a read-model/redis db/database/document-store
