# The Definitive Rust Demo - fe2o3

This repository contains the rather superior rust web server demo.

For information on how to contribute to this industry defining system, please see below.

## Project Repositories

The project uses the following online dashboards and project repositories.

[Project Dashboard](https://travis-ci.com/dashboard)

[Docker Repository](https://cloud.docker.com/repository/docker/thechieftain/fe3o2/general)

[Github Repository](https://github.com/t-h-e-chief/fe2o3)

## Project Build Pipeline

This project is built using a Travis CI build pipeline. A new build is triggered whenever a new change is pushed to the project's Github repository. This makes building and releasing new versions of the application a reliable and straightforward process.

## Docker Usage

To ensure a repeatable process the build is run using a custom docker image, which contains a toolchain built on the [musl libc](https://www.musl-libc.org/how.html) library. This ensures a very compact native compilation of Rust language components on linux systems (our targeted platform.) The docker build compiles the application and runs a basic suite of unit tests. The finished application is then housed in an Alpine linux docker container to ensure the lightest weight image possible.  At the time of writing the complete docker image was 12.5MB in size.

## Pushing a Build

To make a change to the project a developer must create a pull request on a development branch the user has created. A test build will be run on the developer's code and should it successfully pass, the pull request will require an approval from the repository owner. Once an approval is given the pull request can then be merged into the master branch, where a new docker image will be published to docker.io for consumption by our customers.

## Getting started

In order to build and test locally you will need to download and install the following tools:

[Rust Lang](https://www.rust-lang.org/tools/install)
[Docker Engine](https://docs.docker.com/install/)
[Git Client](https://git-scm.com/downloads)

Once you have the toolchain installed, you will need to clone a copy of the repository down to your local system. You can do this via an [SSH](git@github.com:t-h-e-chief/fe2o3.git) connection or via [HTTPS](https://github.com/t-h-e-chief/fe2o3.git)

_But really... who uses HTTPS nowadays?_

## Up and Running

When you are ready to try building and running the application, run the following commands from a terminal session in your project folder.

```
docker build -t thechieftain/fe3o2:latest .
```

This will pull down required docker images and compile the project locally. Note that this can take some time, so please be patient.

```
docker run --rm -p 8080:8080 thechieftain/fe3o2
```

This will then start the docker contain and run the application. In a few moments you should see the following on your terminal session:

```
Http Server running on port 8080.
Press ctrl+c to exit.
```

Testing the application is easy using [Httpie](https://httpie.org/) from your terminal.

`http localhost:8080/status`

This should result in a response similar to the following:

```json
HTTP/1.1 200 OK
content-length: 140
content-type: application/json
date: Sun, 20 Oct 2019 22:57:11 GMT

{
    "fe3o2": [
        {
            "description": "interview technical test",
            "lastcommitsha": "714dcd5",
            "version": "1.0.0"
        }
    ]
}
```

## Need help?

Should you require any help with this project, then please raise an issue in the project's [Github repository](https://github.com/t-h-e-chief/fe2o3/issues).  And we will do our best to help you.

## Contributing

For details on contributing to this repository, please seek medical advice. It is a demo after all...
