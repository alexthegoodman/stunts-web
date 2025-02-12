use crate::components::icon::CreateIcon;
use crate::components::login_form::LoginForm;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <img class="max-w-xs mx-auto mt-2" src="/images/stunts_logo_nobg.png" />
            <LoginForm />
            <section class="container mx-auto px-4 py-6">
                <div class="grid md:grid-cols-3 gap-6">
                    <div class="bg-amber-100/30 p-4 rounded-lg">
                        <div class="bg-red-500/20 w-10 h-10 rounded-md flex items-center justify-center mb-3">
                            <CreateIcon icon="wand".to_string() size="20px".to_string() />
                        </div>
                        <h3 class="text-lg font-semibold mb-1 text-slate-700">
                            Smart Path Generation
                        </h3>
                        <p class="text-gray-700 text-sm">
                            Generate motion paths effortlessly with our intelligent keyframe system.
                        </p>
                    </div>

                    <div class="bg-amber-100/30 p-4 rounded-lg">
                        <div class="bg-red-500/20 w-10 h-10 rounded-md flex items-center justify-center mb-3">
                            <CreateIcon icon="lightning".to_string() size="20px".to_string() />
                        </div>
                        <h3 class="text-lg font-semibold mb-1 text-slate-700">Lightning Fast</h3>
                        <p class="text-gray-700 text-sm">
                            Create animations in minutes with our streamlined workflow.
                        </p>
                    </div>

                    <div class="bg-amber-100/30 p-4 rounded-lg">
                        <div class="bg-red-500/20 w-10 h-10 rounded-md flex items-center justify-center mb-3">
                            <CreateIcon icon="video".to_string() size="20px".to_string() />
                        </div>
                        <h3 class="text-lg font-semibold mb-1 text-slate-700">Video Import</h3>
                        <p class="text-gray-700 text-sm">
                            Import video content seamlessly on Web and Windows.
                        </p>
                    </div>
                </div>
            </section>
        </ErrorBoundary>
    }
}
