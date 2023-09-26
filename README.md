# Rust App

## Usage

To start the http server on port 3000, do

```sh
cargo run
```

To make a request, do

```sh
curl http://localhost:3000
```

To view the api documentation, do

```sh
npx @redocly/cli preview-docs openapi.yaml
```

and follow the instructions. This requires [npm](https://www.npmjs.com) to be
installed. To verify that the implementation follows the documentation, do

```sh
cargo run &
npx specmatic  test --testBaseURL=http://localhost:3000  openapi.yaml
```

This requires [specmatic](https://specmatic.in/getting_started.html) to be
installed.

## Docker

Build the Docker image via
```sh
docker build -f rustapp .
```

Then it needs to be pushed to Dockerhub so that Kubernetes can access it

Running the container locally for testing purposes:
```sh
docker run -p 3000:3000 rustapp
```

It is then accessible via 127.0.0.1:3000.

## Kubernetes

```sh
minikube start --vm-driver=hyperv
```

```sh
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

Access the node via the Minikube VM IP Adress obtained via
```sh
minikube ip
```

Using the Node port found via (under PORTS)
```sh
kubectl get svc
```

Then connect via http://\<Minikube-IP\>:\<NodePort\>