use leptos::*;

#[component]
pub fn Close(
    #[prop(default=20)]
    width: i32,
    #[prop(default=20)]
    height: i32
) -> impl IntoView {
    view! {
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24" width=width height=height>
            <g id="Rounded">
                <line style="fill:none;stroke:#000000;stroke-width:2;stroke-linecap:round;stroke-miterlimit:10;" x1="5" y1="5" x2="19" y2="19"/>
                <line style="fill:none;stroke:#000000;stroke-width:2;stroke-linecap:round;stroke-miterlimit:10;" x1="19" y1="5" x2="5" y2="19"/>
            </g>
       </svg>
    }
}