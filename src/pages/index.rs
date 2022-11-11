use maud::{DOCTYPE, html, Markup};
use crate::pages::partials::{Head,Footer};


pub fn Index(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html{
            (Head("this works ?"))
            body {
                div class="container-fluid"{
                    div class="col" {}
                }
            }
            (Footer("Site Footer"))
        }
    }
}