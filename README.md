# axum-template

<!--toc:start-->
- [axum-template](#axum-template)
  - [What's in the Box](#whats-in-the-box)
  - [Getting Started](#getting-started)
  - [Customizing the Template](#customizing-the-template)
  - [Deploying](#deploying)
<!--toc:end-->

An axum template repository to get your web app off the ground with HTMX,
SQLite, Docker and (Google) OAuth preconfigured

## What's in the Box

We offer some opinionated set up with `axum-template` including the following "tech-stack":

- TailwindCSS
- HTMX
- SQLite
- SQLX
- Docker
- Stripe (for payments)
- Askama (for templates)
- OAuth
  - Google

## Getting Started

To get started, you'll need to have Rust installed. Simply clone the repo and
we've included a development script to get you started:

```shell
./dev.sh
```

This script does a few things:

1. It will install all the necessary dependencies
2. It will check to see if you have a `.env` file if not, it will create one
with the default variables you'll need to run the application.
3. It will create the `database.db` file if you don't already have it.
4. It will run the application in watch mode using `cargo watch -x run`
5. It will also launch tailwind in watch mode

Once you run the script, you can go to `http://localhost:8080` in your browser
to see the application up and running.

## Customizing the Template

Most things are done through environment variables, including:

```env
APP_NAME="your-app-name"
APP_DISPLAY_NAME="Your App Name"
APP_VERSION="123"
APP_PORT="8080"
```

## Deploying

If you have a server, you can deploy using:

```shell
./deploy.sh
```

With that in mind, you can also use similar steps from that script to get to work
on setting up your own pipelines for CI/CD.

### Deploying to DigitalOcean through GHA
You can enable GHA, and use the existing do_deploy.yml which will also
take care of cache-busting through a very rudimentary way (using the `RUN_ID`).

You will need to set up the environment variables in your GitHub Repository:
`DO_USERNAME`, `DO_IP_ADDR`, `DO_SSH_KEY`. This will build the image,
put it on your server (droplet) under the `~/APP_NAME` directory and 
run the container on that droplet.

