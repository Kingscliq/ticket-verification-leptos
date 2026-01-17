use leptos::*;
use crate::api::TicketData;

#[derive(Clone, PartialEq)]
pub enum TicketStatus {
    Valid(TicketData),
    Invalid,
    Used,
    Verifying,
    Error(String),
    Idle,
}

#[component]
pub fn ResultCard<F>(
    status: F
) -> impl IntoView 
where F: Fn() -> TicketStatus + 'static + Clone
{
    let status_data = create_memo(move |_| match status() {
        TicketStatus::Valid(_) => ("status-valid", "✓".to_string(), "Valid Ticket".to_string()),
        TicketStatus::Invalid => ("status-invalid", "✗".to_string(), "Invalid Ticket".to_string()),
        TicketStatus::Used => ("status-used", "!".to_string(), "Already Used".to_string()),
        TicketStatus::Verifying => ("status-idle", "⏳".to_string(), "Verifying...".to_string()),
        TicketStatus::Error(msg) => ("status-invalid", "⚠".to_string(), msg),
        TicketStatus::Idle => ("status-idle", "?".to_string(), "Ready to Scan".to_string()),
    });

    view! {
        <div class={move || format!("result-card {}", status_data.get().0)}>
            <div class="result-icon">{move || status_data.get().1}</div>
            <h2 class="result-title">{move || status_data.get().2}</h2>
        </div>
    }
}
