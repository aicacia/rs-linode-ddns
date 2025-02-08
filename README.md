# linode-ddns

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![API](https://docs.rs/linode-ddns/badge.svg)](https://docs.rs/linode-ddns)
[![Crate](https://img.shields.io/crates/v/linode-ddns.svg)](https://crates.io/crates/linode-ddns)

Linode DDNS


## Docker and Helm

### Deployment

To build and deploy the service using Docker and Helm:

1. **Build the Docker image:**

   ```bash
   # build for x86_64
   docker build -t aicacia/linode-ddns:0.1-x86_64 .

   # build for armv7
   cross build --target armv7-unknown-linux-musleabihf --release
   docker buildx build --no-cache -o type=docker --push --platform linux/arm/v7 --build-arg=TARGET=armv7-unknown-linux-musleabihf -t aicacia/linode-ddns:0.1-armv7 -f Dockerfile.local-target .
   ```

2. **Push the image to the registry:**

   ```bash
   docker push aicacia/linode-ddns:0.1-x86_64
   ```

3. **Deploy with Helm:**

   ```bash
   helm upgrade linode-ddns helm/linode-ddns -n api --install -f values.yaml --set image.hash="$(docker inspect --format='{{index .Id}}' aicacia/linode-ddns:0.1-x86_64)"
   ```

4. **Deploy locally**
   ```bash
   docker run -it \
    -v ${PWD}/config.json:/app/config.json \
    aicacia/linode-ddns:0.1-x86_64
   ```

### Undeployment

To undeploy the service:

```bash
helm delete -n api linode-ddns
```

## Regenerate Linode API Client

```bash
rm -rf linode-api-client && \
npx @openapitools/openapi-generator-cli generate -i https://raw.githubusercontent.com/linode/linode-api-docs/refs/heads/development/openapi.json -g rust -o 'linode-api-client' --additional-properties=packageName=linode-api-client,library=reqwest,avoidBoxedModels=true,enumNameSuffix=Enum
```