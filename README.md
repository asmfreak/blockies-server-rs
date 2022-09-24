# blockies-server

Simple microservice to serve blockies-generated avatars.

## Configuration

To configure microservice, you can use environment variables [from rocket](https://rocket.rs/v0.5-rc/guide/configuration/#overview):

```
ROCKET_PORT=9000 ROCKET_ADDRESS=0.0.0.0 blockies-server
```

## Docker/Podman
You can use a from-scratch container to launch it in docker/podman.
