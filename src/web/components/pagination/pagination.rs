use leptos::*;
use web_sys::MouseEvent;
use crate::web::utils::callback::*;

#[component]
pub fn Pagination(
    #[prop()]
    total_page: i32,
    #[prop()]
    current_page: i32,
    #[prop(default = 10)]
    rows_per_page: i32,
    #[prop()]
    total: i32,
    #[prop(default = noop_callback())]
    on_next: Callback<MouseEvent>,
    #[prop(default = noop_callback())]
    on_prev: Callback<MouseEvent>,
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    let start_index = rows_per_page * (current_page - 1);
    let to_index = [start_index + rows_per_page, total].iter().min().unwrap().clone();

    view! {
        <div class=format!("flex flex-col items-center {class}")>
            <span class="text-sm text-gray-700 dark:text-gray-400">
                Showing <span class="font-semibold text-gray-900 dark:text-white">{start_index}</span> to <span class="font-semibold text-gray-900 dark:text-white">{to_index}</span> of <span class="font-semibold text-gray-900 dark:text-white">{total}</span> Entries
            </span>
            <div class="inline-flex mt-4 xs:mt-0">
                <button class="flex items-center justify-center px-3 h-8 text-sm font-medium text-white bg-gray-800 rounded-s hover:bg-gray-900 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" on:click={on_prev}>
                    Prev
                </button>
                <button class="flex ml-2 items-center justify-center px-3 h-8 text-sm font-medium text-white bg-gray-800 rounded-s hover:bg-gray-900 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" on:click={on_next}>
                    Next
                </button>
            </div>
        </div>
    }
}
