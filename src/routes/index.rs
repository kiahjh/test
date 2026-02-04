use rejoice::{Req, Res, html, island};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req; // Silence unused warning

    res.html(html! {
        h1 { "Welcome to Rejoice!" }
        p { "Click the button below - it's a SolidJS island!" }
        (island!(Counter, { initial: 0 }))
    })
}
