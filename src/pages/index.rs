use maud::{DOCTYPE, html, Markup};
use crate::pages::partials::{Head,Footer};


pub fn Index(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html{
            (Head("this works ?"))
            body {
                div class="container-fluid p-5 d-flex flex-column justify-content-start"{
                    div class="card border-0" {
                        div class="card-body"{
                            div class=""{
                                h2 class="ege-c-blue" {"New Note ++"}
                                textarea class="form-control" style="height:6em" aria-label="With textarea" {}
                                button type="button" class="mt-2 ege-b-white-2 btn w-100 btn-light" {"Add"}
                            }
                        }
                    }
                    div class="ms-auto"{
                        (Footer("Site Footer"))
                    }
                }
            }
        }
    }
}