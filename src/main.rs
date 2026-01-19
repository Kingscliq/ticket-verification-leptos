use leptos::*;
use gloo_storage::{LocalStorage, Storage};

mod components;
mod api;
mod pages;
mod state;

use components::layout::Layout;
// use components::loading::Loading; 
use components::scanner::Scanner;
use components::result::TicketStatus;
use components::login::Login;
use components::reset_password::ResetPassword;
use components::event_selection::EventSelection;

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    EventSelection,
    Scanner,
    ResetPassword,
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    mount_to_body(|| view! { <App/> });
}

use crate::api::{TicketData, ItemDetails, verify_ticket};
use leptos::spawn_local;

#[component]
fn App() -> impl IntoView {
    // Auth State
    let (is_authenticated, set_authenticated) = create_signal(false);
    let (auth_token, set_auth_token) = create_signal(String::new());

    let (current_view, set_current_view) = create_signal(AppView::Scanner);
    let (ticket_status, set_ticket_status) = create_signal(TicketStatus::Idle);
    let (selected_event_id, set_selected_event_id) = create_signal(String::new());
    let (selected_event_title, set_selected_event_title) = create_signal(String::new());

    // Check for existing token on load
    create_effect(move |_| {
         if let Ok(token) = LocalStorage::get::<String>("auth_token") {
             if !token.is_empty() {
                 set_auth_token.set(token);
                 set_authenticated.set(true);
                 
                 // Check if user is stuck in reset flow
                 if let Ok(true) = LocalStorage::get::<bool>("must_reset_password") {
                     set_current_view.set(AppView::ResetPassword);
                 }
             }
         }
    });

    let handle_scan = move |code: String| {
        set_ticket_status.set(TicketStatus::Verifying); 
        // Use current token
        let token = auth_token.get(); 
        
        spawn_local(async move {
            // Try to parse as JSON first to extract ticketCode
            let final_code = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&code) {
                if let Some(ticket_code) = json.get("ticketCode").and_then(|v| v.as_str()) {
                    ticket_code.to_string()
                } else {
                    code
                }
            } else {
                code
            };

            match verify_ticket(final_code, token).await {
                Ok(response) => {
                    let status = if response.success {
                        // We expect data to be present on success
                        if let Some(data) = response.data {
                            TicketStatus::Valid(data)
                        } else {
                            // Fallback if data is missing for some reason, though backend should send it
                            TicketStatus::Error("Ticket valid but missing details".to_string())
                        }
                    } else {
                        TicketStatus::Invalid
                    };
                    set_ticket_status.set(status);
                },
                Err(e) => {
                    let err_msg = if e.contains("404") {
                        "Ticket not found".to_string()
                    } else {
                        e
                    };
                    set_ticket_status.set(TicketStatus::Error(err_msg)); 
                }
            }
        });
    };

    let on_login_success = move |must_reset: bool| {
        if let Ok(token) = LocalStorage::get::<String>("auth_token") {
             set_auth_token.set(token);
             set_authenticated.set(true);
             
             // Persist reset state
             let _ = LocalStorage::set("must_reset_password", must_reset);

             if must_reset {
                 set_current_view.set(AppView::ResetPassword);
             } else {
                 set_current_view.set(AppView::Scanner);
             }
        }
    };

    let on_event_select = move |id: String, title: String| {
        set_selected_event_id.set(id);
        set_selected_event_title.set(title);
        set_current_view.set(AppView::Scanner);
    };

    let on_logout = move |_| {
        let _ = LocalStorage::delete("auth_token");
        let _ = LocalStorage::delete("must_reset_password");
        set_authenticated.set(false);
        set_auth_token.set(String::new());
        set_ticket_status.set(TicketStatus::Idle);
        set_selected_event_id.set(String::new());
        set_selected_event_title.set(String::new());
        set_current_view.set(AppView::EventSelection);
    };

    view! {
        {move || if !is_authenticated.get() {
            view! { <Login on_success=on_login_success /> }.into_view()
        } else {
            view! {
                {move || match current_view.get() {
                    AppView::EventSelection => view! {
                        <Layout>
                            <EventSelection 
                                token=auth_token.get()
                                on_select=on_event_select
                                on_logout=Callback::new(on_logout)
                            />
                        </Layout>
                    }.into_view(),
                    AppView::ResetPassword => view! {
                        <ResetPassword 
                            token=auth_token.get() 
                            on_back=move || set_current_view.set(AppView::Scanner) 
                            on_logout=move || {
                                 let _ = LocalStorage::delete("auth_token");
                                 set_authenticated.set(false);
                                 set_auth_token.set(String::new());
                                 set_ticket_status.set(TicketStatus::Idle);
                            }
                        />
                    }.into_view(),
                    AppView::Scanner => view! {
                        <Layout>
                            <Scanner 
                                on_scan=handle_scan 
                                on_reset=move |_| set_ticket_status.set(TicketStatus::Idle) 
                                status=move || ticket_status.get() 
                            />
                            
                            <div class="scanner-footer">
                                <a href="#" class="footer-link"
                                   on:click=move |ev| {
                                       ev.prevent_default();
                                       set_current_view.set(AppView::ResetPassword);
                                   }
                                >
                                    "Change Password"
                                </a>
                                
                                <button 
                                    class="btn-logout"
                                    on:click=move |_| {
                                            let _ = LocalStorage::delete("auth_token");
                                            set_authenticated.set(false);
                                            set_auth_token.set(String::new());
                                            set_ticket_status.set(TicketStatus::Idle);
                                        }
                                >
                                    "Sign Out"
                                </button>
                            </div>
                        </Layout>
                    }.into_view(),
                }}
            }.into_view()
        }}
    }
}
