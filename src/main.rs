// #![allow(non_snake_case, unused)]
// use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

mod app; 
mod hackernews;

use app::App;



    //  let mut count = use_state(cx, || 0);
    // render! {
    //     h1 { "High-Five counter: {count}" }
    //     button { onclick: move |_| count += 1, "Up high!" }
    //     button { onclick: move |_| count -= 1, "Down low!" }
    // }
//             }
//             div {
//                 dangerous_inner_html: "{comment.text}"
//             }
//             for kid in &comment.sub_comments {
//                 Comment { comment: kid.clone() }
//             }
//         }
//     }
// }


fn main() {
    LaunchBuilder::new(App).launch();
}
