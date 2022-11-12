use maud::{DOCTYPE, html, Markup};

pub fn Head(page_title: &str) -> Markup {
    html! {
        head {
            title { (page_title) }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1"
            script src="https://code.jquery.com/jquery-3.6.1.js" integrity="sha256-3zlB5s2uwoUzrXK3BT7AX3FyvojsraNFxCc2vC/7pNI=" crossorigin="anonymous" {}
            script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM" crossorigin="anonymous" {}
            link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous";
            link href="/public/css/main.css" rel="stylesheet";
            link href="/public/css/ege.css" rel="stylesheet";
            script src="/public/js/index.js" {}
        }
    }
}
