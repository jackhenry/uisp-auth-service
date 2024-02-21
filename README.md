# UISP Authentication Service

### Required Environment Variables

| Name         | Description                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------- |
| `REDIS_URL`  | URL of a redis instance to be used for caching a map between device ip addresses and device ids |
| `UISP_API`   | The hostname of the UISP/UNMS api endpoint.                                                     |
| `UISP_TOKEN` | Auth token used to authorize requests to the UISP API instance                                  |

## Running on Docker

1. Build the docker image
   ```
   docker build -t uisp-auth-service
   ```
2. Create a container with the built image. **Note:** Make sure your Redis instance is reachable from the created container.
   ```
   docker run --name uisp-auth-service -p 8080:8080 --env-file=./env uisp-auth-service
   ```
