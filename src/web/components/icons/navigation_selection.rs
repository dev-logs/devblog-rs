use leptos::*;

#[component]
pub fn NavigationSelection(
    #[prop(default = 1)]
    width: i32,
    #[prop(default = 1)]
    height: i32,
    #[prop(default = "#000")]
    color: &'static str
) -> impl IntoView {
    view! {
        <svg width={format!("{}em", width)} height={format!("{}em", height)} viewBox="0 0 410 307" xmlns="http://www.w3.org/2000/svg">
            <path d="M0 140.707C0 111.988 23.2812 88.7065 52 88.7065H201.103H301.654H322.19C340.616 88.7065 357.666 78.9555 367.009 63.0742L380.965 39.3533L395.482 14.6766L402.741 2.33832V2.33832C404.718 -1.02154 409.868 0.416161 409.819 4.31403L406.03 303.197C405.982 307.021 400.844 308.223 399.104 304.819V304.819L392.365 291.637L378.886 265.275L366.413 240.878C357.523 223.491 339.642 212.55 320.113 212.55H301.654H201.103H52C23.2812 212.55 0 189.268 0 160.55V140.707Z" fill={color}/>
        </svg>
    }
}