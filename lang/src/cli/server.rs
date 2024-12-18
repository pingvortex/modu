use rouille::router;
use crate::utils;
use crate::parser::parse;
use std::sync::Arc;

pub fn server() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut port = 2424;

    if args.len() > 2 {
        port = args[2].parse::<u16>().unwrap();
    }

    println!("Modu server starting on port {}", port);

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("Modu interpreter server is running")
            },

            (POST) (/eval) => {
                println!("POST /eval | {} | {}", request.remote_addr(), request.header("User-Agent").unwrap_or("unknown"));

                let text = rouille::try_or_400!(rouille::input::plain_text_body(request));

                let context = &mut utils::create_context();

                std::io::set_output_capture(Some(Default::default()));

                parse(&text, context).unwrap_or_else(|e| {
                    println!("\n⚠️  {}", e.0);
                    println!("Traceback (most recent call last):");
                    println!("    File \"<stdin>\", line 1");
                });

                let captured = String::from_utf8(
                    Arc::try_unwrap(
                        std::io::set_output_capture(None).unwrap()
                    )
                        .unwrap()
                        .into_inner()
                        .unwrap()
                ).unwrap();

                rouille::Response::text(captured)
            },

            _ => {
                rouille::Response::empty_404()
            }
        )
    });
}