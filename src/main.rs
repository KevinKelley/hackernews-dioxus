#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {

    // let title = "title";
    // let by = "author";
    // let score = 0;
    // let time = chrono::Utc::now();
    // let comments = "comments";

    // render! {
    //     "{title} by {by} ({score}) {time} {comments}"
    // }

     let mut count = use_state(cx, || 0);


    render! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}

