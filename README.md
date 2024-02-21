# UISP Authentication Service

### Required Environment Variables

| Name         | Description                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------- |
| `REDIS_URL`  | URL of a redis instance to be used for caching a map between device ip addresses and device ids |
| `UISP_API`   | The hostname of the UISP/UNMS api endpoint.                                                     |
| `UISP_TOKEN` | Auth token used to authorize requests to the UISP API instance                                  |
