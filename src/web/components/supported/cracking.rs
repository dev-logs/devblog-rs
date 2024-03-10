use leptos::*;

#[component]
pub fn CrackingBackground(
    #[prop(default = "250px")]
    width: &'static str,
    #[prop(default = "100%")]
    height: &'static str
) -> impl IntoView {
   view! {
       <svg width=width height=height fill="none" xmlns="http://www.w3.org/2000/svg">
           <g filter="url(#filter0_d_1_2)">
               <path d="M1430 254L34 254L34 52.9998C34 52.9998 335.281 203.735 339.904 201.964C416.34 172.698 471.433 119.32 471.433 119.32C471.433 119.32 718.548 185.64 718.548 192.782C718.548 199.924 1039.4 94.8323 1039.4 94.8323L1098.19 165.233L1199.82 172.322L1430 52.9998L1430 254Z" fill="#030712" fill-opacity="0.95" shape-rendering="crispEdges"/>
               <path d="M2826 254L1430 254L1430 52.9998C1430 52.9998 1731.28 203.735 1735.9 201.965C1812.34 172.698 1867.43 119.32 1867.43 119.32C1867.43 119.32 2114.55 185.64 2114.55 192.782C2114.55 199.924 2435.4 94.8325 2435.4 94.8325L2494.19 165.234L2595.82 172.323L2826 53L2826 254Z" fill="#030712" fill-opacity="0.95" shape-rendering="crispEdges"/>
           </g>
           <defs>
                <filter id="filter0_d_1_2" x="0.799999" y="0.799847" width="2858.4" height="267.4" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                     <feFlood flood-opacity="0" result="BackgroundImageFix"/>
                     <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
                     <feOffset dy="-19"/>
                     <feGaussianBlur stdDeviation="16.6"/>
                     <feComposite in2="hardAlpha" operator="out"/>
                     <feColorMatrix type="matrix" values="0 0 0 0 0.520101 0 0 0 0 0.520101 0 0 0 0 0.520101 0 0 0 0.25 0"/>
                     <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_1_2"/>
                     <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_1_2" result="shape"/>
                </filter>
           </defs>
       </svg>
   }
}
