use leptos::*;
use gloo_storage::{LocalStorage, Storage};
use crate::api::{reset_password, ResetPasswordRequest};

#[component]
pub fn ResetPassword<F, F2>(
    token: String,
    on_back: F,
    on_logout: F2,
) -> impl IntoView 
where 
    F: Fn() + Clone + 'static,
    F2: Fn() + Clone + 'static
{
    let (current_password, set_current_password) = create_signal(String::new());
    let (new_password, set_new_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    
    let (error_msg, set_error_msg) = create_signal(String::new());
    let (success_msg, set_success_msg) = create_signal(String::new());
    let (show_current, set_show_current) = create_signal(false);
    let (show_new, set_show_new) = create_signal(false);
    let (show_confirm, set_show_confirm) = create_signal(false);
    let (loading, set_loading) = create_signal(false);

    let handle_reset = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let cp = current_password.get();
        let np = new_password.get();
        let cnp = confirm_password.get();

        if cp.is_empty() || np.is_empty() || cnp.is_empty() {
            set_error_msg.set("All fields are required".to_string());
            return;
        }

        if np != cnp {
            set_error_msg.set("New passwords do not match".to_string());
            return;
        }

        // Basic validation matching user requirements
        if np.len() < 8 {
            set_error_msg.set("New password must be at least 8 characters".to_string());
            return;
        }

        set_loading.set(true);
        set_error_msg.set(String::new());
        set_success_msg.set(String::new());

        let token_clone = token.clone();
        let req = ResetPasswordRequest {
            current_password: cp,
            new_password: np,
            confirm_new_password: cnp,
        };

        spawn_local(async move {
            match reset_password(req, token_clone).await {
                Ok(_) => {
                    set_success_msg.set("Password reset successfully!".to_string());
                    set_current_password.set(String::new());
                    set_new_password.set(String::new());
                    set_confirm_password.set(String::new());
                    // Clear the reset flag
                    let _ = LocalStorage::delete("must_reset_password");
                },
                Err(e) => {
                    set_error_msg.set(e);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="login-container">
            <div class="login-card">
                <div class="logo-container">
                    <img src="public/logo.svg" alt="Gidigroove" class="logo-img" />
                </div>
                
                <h1 class="login-title">"Reset Password"</h1>
                <p class="login-subtitle">"Update your account credentials"</p>

                <form on:submit=handle_reset class="login-form">
                    {move || {
                        let success = success_msg.get();
                        if !success.is_empty() {
                            let on_logout = on_logout.clone();
                            view! {
                                <div class="alert-success">{success}</div>
                                <button type="button" class="btn btn-primary w-full mt-4" on:click=move |_| on_logout()>
                                    "Sign In"
                                </button>
                            }.into_view()
                        } else {
                            let on_back = on_back.clone();
                            view! {
                                <div class="form-group">
                                    <label for="current_password">"Current Password"</label>
                                    <div class="password-wrapper relative">
                                        <input 
                                            type=move || if show_current.get() { "text" } else { "password" }
                                            id="current_password"
                                            class="input-field"
                                            placeholder="Current Password"
                                            prop:value=current_password
                                            on:input=move |ev| set_current_password.set(event_target_value(&ev))
                                            disabled=loading
                                        />
                                        <button 
                                            type="button"
                                            class="password-toggle absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                                            on:click=move |_| set_show_current.update(|v| *v = !*v)
                                        >
                                            {move || if show_current.get() { "Hide" } else { "Show" }}
                                        </button>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="new_password">"New Password"</label>
                                    <div class="password-wrapper relative">
                                        <input 
                                            type=move || if show_new.get() { "text" } else { "password" }
                                            id="new_password"
                                            class="input-field"
                                            placeholder="New Password (min 8 chars)"
                                            prop:value=new_password
                                            on:input=move |ev| set_new_password.set(event_target_value(&ev))
                                            disabled=loading
                                        />
                                        <button 
                                            type="button"
                                            class="password-toggle absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                                            on:click=move |_| set_show_new.update(|v| *v = !*v)
                                        >
                                            {move || if show_new.get() { "Hide" } else { "Show" }}
                                        </button>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="confirm_password">"Confirm New Password"</label>
                                    <div class="password-wrapper relative">
                                        <input 
                                            type=move || if show_confirm.get() { "text" } else { "password" }
                                            id="confirm_password"
                                            class="input-field"
                                            placeholder="Confirm New Password"
                                            prop:value=confirm_password
                                            on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                            disabled=loading
                                        />
                                        <button 
                                            type="button"
                                            class="password-toggle absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                                            on:click=move |_| set_show_confirm.update(|v| *v = !*v)
                                        >
                                            {move || if show_confirm.get() { "Hide" } else { "Show" }}
                                        </button>
                                    </div>
                                </div>

                                {move || {
                                    let err = error_msg.get();
                                    if !err.is_empty() {
                                        view! { <div class="alert-error">{err}</div> }.into_view()
                                    } else {
                                        view! { <div class="h-4"></div> }.into_view()
                                    }
                                }}

                                <button type="submit" class="btn btn-primary w-full mt-4" disabled=loading>
                                    {move || if loading.get() { "Resetting..." } else { "Reset Password" }}
                                </button>
                                
                                <div class="text-center mt-4">
                                    <a href="#" class="forgot-link text-sm text-primary hover:underline"
                                       on:click=move |ev| {
                                           ev.prevent_default();
                                           on_back();
                                       }
                                    >
                                        "Back to Scanner"
                                    </a>
                                </div>
                            }.into_view()
                        }
                    }}
                </form>
            </div>
        </div>
    }
}
