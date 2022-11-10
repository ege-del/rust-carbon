use maud::{DOCTYPE, html, Markup};

pub fn partial(text: &str) -> Markup {
    html! {
        div class="d-flex justify-content-center" {
            p { (text) }
        }
    }
}
