use leptos::*;

#[component]
pub fn Product(
    #[prop(default = 1)]
    width: i32,
    #[prop(default = 1)]
    height: i32,
    #[prop(default = "#000")]
    color: &'static str
) -> impl IntoView {
    view! {
        <svg width={format!("{}em", width)} height={format!("{}em", height)} viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg">
            <path fill={color} d="M4 4h6v6H4z" className="clr-i-solid--badged clr-i-solid-path-1--badged"></path>
            <path fill={color} d="M4 15h6v6H4z" className="clr-i-solid--badged clr-i-solid-path-2--badged"></path>
            <path fill={color} d="M4 26h6v6H4z" className="clr-i-solid--badged clr-i-solid-path-3--badged"></path>
            <path fill={color} d="M15 4h6v6h-6z" className="clr-i-solid--badged clr-i-solid-path-4--badged"></path>
            <path fill={color} d="M15 15h6v6h-6z" className="clr-i-solid--badged clr-i-solid-path-5--badged"></path>
            <path fill={color} d="M15 26h6v6h-6z" className="clr-i-solid--badged clr-i-solid-path-6--badged"></path>
            <path fill={color} d="M26 15h6v6h-6z" className="clr-i-solid--badged clr-i-solid-path-7--badged"></path>
            <path fill={color} d="M26 26h6v6h-6z" className="clr-i-solid--badged clr-i-solid-path-8--badged"></path>
            <circle cx="30" cy="6" r="5" fill={color} className="clr-i-solid--badged clr-i-solid-path-9--badged clr-i-badge"></circle>
            <path fill="none" d="M0 0h36v36H0z"></path>
        </svg>
    }
}