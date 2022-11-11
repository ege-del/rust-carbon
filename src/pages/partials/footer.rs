use maud::{DOCTYPE, html, Markup};

pub fn Footer(text: &str) -> Markup {
    html! {
        div class="d-flex justify-content-center" {
            p { (text) }
        }
    }
}
