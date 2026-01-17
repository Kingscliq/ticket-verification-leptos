use leptos::*;
use gloo_storage::{LocalStorage, Storage};
use crate::api::{login, LoginRequest};

#[component]
pub fn Login<F>(
    on_success: F
) -> impl IntoView 
where F: Fn(bool) + 'static + Clone
{
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(String::new());
    let (info_msg, set_info_msg) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);
    let (show_password, set_show_password) = create_signal(false);

    let on_success_login = on_success.clone();
    let handle_login = move |_| {
        let e = email.get();
        let p = password.get();

        if e.is_empty() || p.is_empty() {
             set_error_msg.set("Please fill in all fields".to_string());
             return;
        }

        set_loading.set(true);
        set_error_msg.set(String::new());
        set_info_msg.set(String::new());
        
        let on_success_cb = on_success_login.clone();

        spawn_local(async move {
             let req = LoginRequest { email: e, password: p };
             match login(req).await {
                 Ok(res) => {
                     // Save token
                     let _ = LocalStorage::set("auth_token", res.access_token);
                     set_loading.set(false);
                     on_success_cb(res.must_reset_password);
                 },
                 Err(err) => {
                     set_error_msg.set(err);
                     set_loading.set(false);
                 }
             }
        });
    };


    view! {
        <div class="login-container">
             <div class="login-card">
                 <div class="logo-container">
                     <img src="public/logo.svg" alt="Gidigroove" class="logo-img" />
                 </div>
                 
                 <h1 class="login-title">"Welcome back"</h1>
                 <p class="login-subtitle">"You should be able to verify ticket status from here"</p>

                 <div class="login-form">
                     <div class="form-group">
                         <label>"Email Address" <span class="text-red-500">"*"</span></label>
                         <input 
                             type="email" 
                             class="input-field" 
                             prop:value=email
                             on:input=move |ev| set_email.set(event_target_value(&ev))
                         />
                     </div>
                     
                     <div class="form-group">
                         <label>"Password"</label>
                         <div class="password-wrapper relative">
                             <input 
                                 type=move || if show_password.get() { "text" } else { "password" }
                                 class="input-field" 
                                 prop:value=password
                                 on:input=move |ev| set_password.set(event_target_value(&ev))
                             />
                             <button 
                                 class="password-toggle absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                                 on:click=move |_| set_show_password.update(|v| *v = !*v)
                             >
                                 {move || if show_password.get() { "Hide" } else { "Show" }}
                             </button>
                         </div>
                     </div>
                     
                     <div class="text-right">
                         <a href="#" class="forgot-link text-sm text-primary hover:underline"
                            on:click=move |_| set_info_msg.set("Please contact the administrator to reset your password.".to_string())
                         >
                            "Forgot password?"
                         </a>
                     </div>

                     {move || {
                         let msg = info_msg.get();
                         if !msg.is_empty() {
                             view! {
                                 <div class="alert-info">
                                     {msg}
                                 </div>
                             }.into_view()
                         } else {
                             view! { <div class="hidden"></div> }.into_view()
                         }
                     }}

                     {move || {
                         let msg = error_msg.get();
                         if !msg.is_empty() {
                             view! {
                                 <div class="alert-error">
                                     {msg}
                                 </div>
                             }.into_view()
                         } else {
                             view! { <div class="h-4"></div> }.into_view()
                         }
                     }}
                     
                     <button 
                         class="btn btn-primary w-full mt-4"
                         disabled=move || loading.get()
                         on:click=handle_login
                     >
                         {move || if loading.get() { "Signing In..." } else { "Sign In" }}
                     </button>
                     
                 </div>
             </div>
        </div>
    }
}
