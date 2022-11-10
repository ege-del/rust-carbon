use maud::{DOCTYPE, html, Markup};
use crate::pages::partials::{head,footer};


pub fn Page(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html{
            (head::partial("this works ?"))
            body {
                div class="container-fluid"{
                    div class="col" {}
                }
            }
            (footer::partial("Site Footer"))
        }
    }
}