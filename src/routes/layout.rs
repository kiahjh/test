use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    let _ = req; // Silence unused warning

    res.html(html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome" }
            }
            body {
                (children)
            }
        }
    })
}
