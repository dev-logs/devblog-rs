use leptos::*;

#[component]
pub fn At(
    #[prop(default = 1)]
    width: i32,
    #[prop(default = 1)]
    height: i32,
    #[prop(default = "#000")]
    color: &'static str
) -> impl IntoView {
    view! {
        <svg width={format!("{}em", width)} height={format!("{}em", height)} viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
           <path fill={color} d="M992 704q-29 64-96 64H736q-25 0-43-16.5T673 711q-71 57-161 57q-106 0-181-75t-75-181t75-181t181-75q99 0 173 67q10-3 19-3q27 0 45.5 19t18.5 45v256h105q23-64 23-128q0-104-51.5-192.5t-140-140T512 128t-192.5 51.5t-140 140T128 512t51.5 192.5t140 140T512 896h192q27 0 45.5 18.5T768 960t-18.5 45.5T704 1024H512q-104 0-199-40.5t-163.5-109T40.5 711T0 512t40.5-199t109-163.5T313 40.5T512 0t199 40.5t163.5 109t109 163.5t40.5 199q0 62-6.5 105T992 704M512 384q-53 0-90.5 37.5T384 512t37.5 90.5T512 640t90.5-37.5T640 512t-37.5-90.5T512 384"></path>
        </svg>
    }
}