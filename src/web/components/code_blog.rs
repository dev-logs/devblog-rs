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
            <div class="rounded-xl border border-stone-800" style:padding="0px">
                <pre class="rounded-xl" style:padding="0px" style:padding-left="20px" style:padding="15px" style:margin="5px" style:border="none" style:box-shadow="none">
                    <code class=format!("w-full rounded-xl overflow-hidden scroll-auto font-code language-{language}")>
                      {final_value}
                    </code>
                </pre>
            </div>
        </div>
    }
}
