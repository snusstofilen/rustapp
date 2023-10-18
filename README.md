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

Then it needs to be pushed to Dockerhub so that Kubernetes can access it.

To use a local docker image in minikube, do the following:

```sh
minikube start
eval $(minikube -p minikube docker-env) # Use minikube's docker environment when executing docker commands
docker build -t hiqelias/rustapp:latest .
kubectl apply -f deployment.yaml

# Set the imagePullPolicy of the deployment to Never
# This means that kubernetes will not look for images on the web
kubectl edit deployment # Search for imagePullPolicy
kubectl apply -f service.yaml
minikube service rustapp-service --url

# In another terminal, use the URL from the command above as the base URL for accessing the API
curl http://<url-from-previous-command>/post_example
```

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

## PostgreSQL

Deploy the PV:
kubectl apply -f db-persistent-volume.yaml
Deploy the PVC
kubectl apply -f db-volume-claim.yaml
The environment variables are needed by the cluster. Deploy them as follows:
kubectl apply -f db-configmap.yaml
Next, create the deployment and add pods replicas.
kubectl apply -f db-deployment.yaml
Finally run the service to expose the cluster
kubectl apply -f db-service.yaml