# UISP Authentication Service

A self-hostable service which provides redirects to the management interface of a ubiquiti device monitored through Ubiquiti's Network Monitoring Service (UNMS/UISP). When redirected, device management sessions are pre-authenticated.

## Why?

Typically, when accessing a ubiquiti device, you must provide device credentials to access the management interface. This can be annoying when you manage hundreds/thousands of different ubiquiti antennas, routers, switches, etc..

It is possible to enter pre-authenticated management sessions when you access a device through the UNMS/UISP web app. However, I personally dislike the interface and find the process to be clunky and slow.

This service allows you to directly link to a Ubiquiti device's management interface on any platform. For example, you can add a management url to a Ubiquiti device on a device page within LibreNMS.

## How it works

This service exposes a single endpoint:

```
/redirect/{ip address}
```

Sending a GET request to this endpoint will result in a HTTP redirect to the management interface of the device with the provided IP address.

## Running the Service

### 1. Clone the repo

```
git clone https://github.com/jackhenry/uisp-auth-service.git
cd uisp-auth-service
```

### 2. Populate the required environment variables.

> Use the file `.env.example` as a template

```
mv .env.example .env
nano .env
```

> Description of required environmnet variables:

| Name         | Description                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------- |
| `REDIS_URL`  | URL of a redis instance to be used for caching a map between device ip addresses and device ids |
| `UISP_API`   | The hostname of the UISP/UNMS api endpoint                                                      |
| `UISP_TOKEN` | Auth token used to authorize requests to the UISP API instance                                  |

### 3. Build and Run

`With Cargo`

```
cargo build --release
export $(cat .env | xargs) && ./target/release/uisp-auth-service
```

`With Docker`

```
docker build -t uisp-auth-service
docker run --name uisp-auth-service -p 8080:8080 --env-file=./env uisp-auth-service
```
