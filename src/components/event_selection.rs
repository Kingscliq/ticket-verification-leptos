use leptos::*;
use crate::api::{Listing, fetch_listings};

#[component]
pub fn EventSelection<F>(
    token: String,
    on_select: F,
    on_logout: Callback<()>,
) -> impl IntoView
where
    F: Fn(String, String) + 'static,
{
    let (listings, set_listings) = create_signal(Vec::<Listing>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Store on_select to allow it to be called multiple times
    let on_select_stored = store_value(on_select);

    // Fetch listings on mount
    {
        let token_clone = token.clone();
        create_effect(move |_| {
            let token_value = token_clone.clone();
            spawn_local(async move {
                set_loading.set(true);
                match fetch_listings(token_value).await {
                    Ok(data) => {
                        set_listings.set(data);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                        set_loading.set(false);
                    }
                }
            });
        });
    }

    view! {
        <div class="event-selection-container">
            <div class="event-selection-header">
                <h2>"Select Event"</h2>
                <p class="event-selection-subtitle">"Choose an event to verify tickets for"</p>
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="loading-state">
                            <p>"Loading events..."</p>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    let token_for_retry = token.clone();
                    view! {
                        <div class="error-state">
                            <p class="error-msg">{err}</p>
                            <button 
                                class="btn-primary"
                                on:click=move |_| {
                                    set_error.set(None);
                                    set_loading.set(true);
                                    let token_clone = token_for_retry.clone();
                                    spawn_local(async move {
                                        match fetch_listings(token_clone).await {
                                            Ok(data) => {
                                                set_listings.set(data);
                                                set_loading.set(false);
                                            }
                                            Err(e) => {
                                                set_error.set(Some(e));
                                                set_loading.set(false);
                                            }
                                        }
                                    });
                                }
                            >
                                "Retry"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    let listings_clone = listings.get();
                    
                    if listings_clone.is_empty() {
                        view! {
                            <div class="empty-state">
                                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="empty-state-icon">
                                    <rect width="18" height="18" x="3" y="4" rx="2" ry="2"/>
                                    <line x1="16" x2="16" y1="2" y2="6"/>
                                    <line x1="8" x2="8" y1="2" y2="6"/>
                                    <line x1="3" x2="21" y1="10" y2="10"/>
                                </svg>
                                <h3 class="empty-state-title">"No Events Found"</h3>
                                <p class="empty-state-message">"You don't have any events to verify tickets for at the moment."</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="event-list-container">
                                {listings_clone.iter().map(|listing| {
                                    let id = listing.id.clone();
                                    let title = listing.title.clone();
                                    let title_for_display = title.clone();
                                    let listing_type = listing.listing_type.clone();
                                    
                                    // Capitalize first letter
                                    let type_display = if !listing_type.is_empty() {
                                        let mut c = listing_type.chars();
                                        match c.next() {
                                            None => String::new(),
                                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                        }
                                    } else {
                                        String::new()
                                    };

                                    let date = listing.date.clone();
                                    view! {
                                        <div 
                                            class="event-card"
                                            on:click=move |_| {
                                                on_select_stored.with_value(|f| f(id.clone(), title.clone()));
                                            }
                                        >
                                            <div class="event-card-content">
                                                <h3 class="event-card-title">{title_for_display.clone()}</h3>
                                                <div class="event-card-meta">
                                                    <span class="event-card-type">{type_display}</span>
                                                    {(!date.is_empty()).then(|| view! {
                                                        <span class="event-card-date">{date.clone()}</span>
                                                    })}
                                                </div>
                                            </div>
                                            <div class="event-card-arrow">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="m9 18 6-6-6-6"/>
                                                </svg>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_view()
                    }
                }
            }}

            <div class="event-selection-footer">
                <button 
                    class="btn-logout"
                    on:click=move |_| on_logout.call(())
                >
                    "Sign Out"
                </button>
            </div>
        </div>
    }
}
