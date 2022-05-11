# Basic Warp API

diesel cli
warp


Heroku deploy

Procfile->
web ./target/release/family-api


RustConfig->
VERSION=nightly


heroku buildpacks->
heroku create --buildpack emk/rust
heroku buildpacks:set emk/rust

Random String using node
require('crypto').randomBytes(64).toString('hex')
