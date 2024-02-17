use leptos::*;

#[component]
pub fn CodeBlock(
    #[prop(default = "")]
    language: &'static str,
    #[prop(default = "")]
    code: &'static str
) -> impl IntoView {
    let final_value = if language == "sh" || language == "bash" {
        format!("$ {}", code.trim().replace("\n", "\n$ "))
    }
    else {
        code.trim().to_string()
    };

    view! {
        <div class="w-full rounded-lg">
            <div class="rounded-2xl" style:padding="0px">
                <pre class="p-1 rounded-3xl" style:padding="0px" style:padding-left="20px" style:padding="10px" style:border="none" style:box-shadow="none">
                    <code class=format!("w-full rounded-2xl overflow-hidden scroll-auto font-code language-{language}")>
                      {final_value}
                    </code>
                </pre>
            </div>
        </div>
    }
}
