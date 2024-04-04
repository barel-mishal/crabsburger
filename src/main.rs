#![allow(non_snake_case)]
mod app;
mod components;
mod model;
mod repository;
mod route;
use dioxus::prelude::*;
mod utils;

fn main() {
    launch(app::App);
}
