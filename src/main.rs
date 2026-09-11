//! Classeur d'informatique L1 — application Leptos rendue côté client.

mod app;
mod components;
mod convert;
mod course;
mod pages;
mod practice;
mod prompts;
mod scroll;
mod search;
mod theme;

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    register_service_worker();
    mount_to_body(app::App);
}

/// Enregistre le service worker qui garde le classeur consultable hors ligne.
///
/// Désactivé en debug : il servirait un ancien bundle pendant le développement.
#[cfg(not(debug_assertions))]
fn register_service_worker() {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().service_worker().register("./sw.js");
    }
}

#[cfg(debug_assertions)]
fn register_service_worker() {}
