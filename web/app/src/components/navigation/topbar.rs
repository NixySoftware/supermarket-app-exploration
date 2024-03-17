use leptos::*;
use leptos_router::A;

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="border-b">
                <div class="flex h-16 items-center px-4">
                    <div class="flex items-center space-x-2">
                        <A href="/" class="hover:text-primary text-md font-medium transition-colors">
                            Supermarket
                        </A>
                    </div>

                    <div class="ml-auto flex items-center space-x-4">
                        <A href="/auth/sign-in" class="text-muted-foreground hover:text-primary text-sm font-medium transition-colors">
                            Sign in
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}
