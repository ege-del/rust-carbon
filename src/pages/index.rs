use maud::{DOCTYPE, html, Markup};
use crate::pages::partials::{Head,Footer};


pub fn Index(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html{
            (Head("this works"))
            body class="flex-c" {
                div class="container-fluid"{
                    div class="row d-flex justify-content-center"{
                        div class="col-12 col-sm-9 col-md-7 h-100 "{
                            div class="card border-0" {
                            div class="card-body"{
                                div class=""{
                                    h2 class="ege-c-blue" {"New Note"}
                                    textarea id="input_text" class="form-control" style="height:6em" aria-label="With textarea" {}
                                    button id="send_btn" type="button" class="mt-2 ege-b-white-2 btn w-100 btn-light" {"Add"}
                                    }
                                }
                            }
                            div id="note-container"{

                            }
                        }
                    }
                }
                footer class="d-flex justify-content-center"{
                    (Footer("Rust Carbon"))
                }
            }
        }
    }
}