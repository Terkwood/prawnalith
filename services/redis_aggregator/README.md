# prawnalithic 🦐 redis aggregator

This small service which listens for relevant updates to the redis datastore, aggregates them, and periodically pushes them to google's cloud via its basic pub/sub system.  

It also exposes a quick & dirty cloning facility which makes sure that the google cloud replicant is in sync with the local datastore every time the service is initialized ⚠️

There are a couple of different projects used to support this effort:

- *redis_delta* - which is a simple serialization strategy for capturing relevant prawnlike 🦐 updates to the local site's redis database
- *gcloud_push* - which handles listening for such updates and pushing them to google's pub/sub system.  It also pushes the entire set of relevant data up to GCP on startup.
