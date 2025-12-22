//! Main application component.

use yew::prelude::*;

use crate::components::canvas::Canvas;
use crate::components::dialog::render_dialog;
use crate::components::toolbox::Toolbox;
use crate::state::{AppStateContext, AppStateProvider};

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppStateProvider>
            <AppContent />
        </AppStateProvider>
    }
}

#[function_component(AppContent)]
fn app_content() -> Html {
    let state = use_context::<AppStateContext>().expect("AppStateContext not found");
    html! {
        <div class="app">
            <Toolbox />
            <Canvas />
            {render_dialog(&state)}
        </div>
    }
}
