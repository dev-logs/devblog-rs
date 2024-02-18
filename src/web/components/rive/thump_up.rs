use leptos::*;
use web_sys::js_sys::eval;

#[component]
pub fn ThumbUpRive() -> impl IntoView {
    create_effect(move |_| {
        eval(r###"
            // Function to load script
            const loadScript = (url, callback) => {
                const script = document.createElement('script');
                script.src = url;
                script.onload = callback;
                document.body.appendChild(script);
            };

            // Execute script after loading
            const executeScript = () => {
                const r = new rive.Rive({
                    src: "/assets/riv/rive.riv",
                    color: '#000000',
                    canvas: document.getElementById("canvas"),
                    autoplay: true,
                    stateMachines: "thumb_up",
                    onLoad: () => {
                        r.resizeDrawingSurfaceToCanvas();
                    },
                });
            };

            // Load script
            loadScript("https://unpkg.com/@rive-app/canvas@2.7.0", executeScript);
        "###)
    });

    view! {
        <div>
            <canvas id="canvas" width="500" height="500"></canvas>
        </div>
    }
}
