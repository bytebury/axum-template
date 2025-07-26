# axum-template
An axum template repository to get your web app off the ground with HTMX, SQLite, and OAuth preconfigured

## What's in the Box
We offer some opinionated set up with `axum-template` including the following "tech-stack":

* TailwindCSS
* HTMX
* SQLite
* SQLX
* Docker
* Askama (for templates)
* OAuth
  * Google

## Getting Started
To get started, you'll need to have Rust installed. Simply clone the repo and we've included
a development script to get you started:

```shell
./dev.sh
```

This script does a few things:
1. It will check to see if you have a `.env` file if not, it will create one with the default variables you'll need to run the application.
2. It will create the `database.db` file if you don't already have it.
3. It will run the application in watch mode using `cargo watch -x run`

Once you run the script, you can go to `http://localhost:8080` in your browser to see the application up and running.

## Customizing the Template
If you are looking to customize the template, e.g. change the name from "Axum Template" to something reasonable, you can do that in the code itself.

Application Details are all configured in the `lib.rs#initialize_app` function. You may change them to your liking.

## Deploying
If you have a server, you can deploy using:

```shell
./deploy.sh
```

With that in mind, you can also use similar steps from that script to get to work on setting up your own pipelines for CI/CD.
