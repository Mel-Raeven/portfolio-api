# Portfolio CMS

## Description

Hey! this projects servers as an CMS / API for my Portfolio website: https://melraeven.nl. The goal is to make a seperate app that allows me to commit new projects to a DB so i dont have to touch my front end anymore.

## Tech

I'm using Rust to build this project. Also using some crates:

- Diesel
- Rocket

Also running a PostgreSQL database to store the projects.

## Docker

for now the DB can be rand with:
`docker-compose up -d`

use the -d flag to run in background

app can be build by:
`cargo run`

This will be added to a dockerfile so the compose command can build everything :)

## TODO!

- build project in dockerfile
- CRUD (already implemented CR)
- Electron frontend? not sure about this tho
