use leptos::*;
use crate::api::VerifiedTicket;

#[component]
pub fn VerifiedTicketDetails<F>(
    ticket_id: String,
    on_back: F,
) -> impl IntoView 
where
    F: Fn() + 'static,
{
    // Dummy Data Lookup (would normally fetch from API or Store)
    let ticket = match ticket_id.as_str() {
        "1" => Some(VerifiedTicket {
            id: "1".to_string(),
            ticket_code: "TIK-123456".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "John Doe".to_string(),
            ticket_type: "VIP Admission".to_string(),
            verified_at: "10:30 AM".to_string(),
            status: "Valid".to_string(),
        }),
        "2" => Some(VerifiedTicket {
            id: "2".to_string(),
            ticket_code: "TIK-789012".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Jane Smith".to_string(),
            ticket_type: "Regular Admission".to_string(),
            verified_at: "10:45 AM".to_string(),
            status: "Valid".to_string(),
        }),
        "3" => Some(VerifiedTicket {
            id: "3".to_string(),
            ticket_code: "TIK-345678".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Mike Johnson".to_string(),
            ticket_type: "Early Bird".to_string(),
            verified_at: "11:00 AM".to_string(),
            status: "Valid".to_string(),
        }),
        "4" => Some(VerifiedTicket {
            id: "4".to_string(),
            ticket_code: "TIK-901234".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Sarah Williams".to_string(),
            ticket_type: "VIP Admission".to_string(),
            verified_at: "11:15 AM".to_string(),
            status: "Invalid".to_string(),
        }),
        _ => None,
    };

    view! {
        <div class="verified-details-container">
            <div class="verified-header">
                <button class="btn-back" on:click=move |_| on_back()>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="m15 18-6-6 6-6"/>
                    </svg>
                    "Back"
                </button>
                <h2>"Ticket Details"</h2>
            </div>

            {if let Some(t) = ticket {
                view! {
                    <div class="ticket-details-card">
                        <div class="details-status" data-status=t.status.to_lowercase()>
                            {if t.status == "Valid" {
                                view! { 
                                    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="M22 4 12 14.01l-3-3"/></svg>
                                    <span>"Verified Valid"</span>
                                }
                            } else {
                                view! { 
                                    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>
                                    <span>"Invalid Ticket"</span>
                                }
                            }}
                        </div>

                        <div class="details-group">
                            <label>"Attendee"</label>
                            <p class="details-value large">{t.attendee_name}</p>
                        </div>

                        <div class="details-row">
                            <div class="details-group">
                                <label>"Ticket Code"</label>
                                <p class="details-value">{t.ticket_code}</p>
                            </div>
                            <div class="details-group">
                                <label>"Verified At"</label>
                                <p class="details-value">{t.verified_at}</p>
                            </div>
                        </div>

                        <div class="details-group">
                             <label>"Ticket Type"</label>
                             <div class="ticket-type-badge">{t.ticket_type}</div>
                        </div>

                        <div class="details-group">
                            <label>"Event"</label>
                            <p class="details-value">{t.event_name}</p>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="error-state">
                        "Ticket not found"
                    </div>
                }.into_view()
            }}
        </div>
    }
}
