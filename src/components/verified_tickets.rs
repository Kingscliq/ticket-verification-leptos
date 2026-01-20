use leptos::*;
use crate::api::VerifiedTicket;

#[component]
pub fn VerifiedTicketsList<F>(
    on_back: Callback<()>,
    on_select_ticket: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone,
{
    // Dummy Data
    let tickets = vec![
        VerifiedTicket {
            id: "1".to_string(),
            ticket_code: "TIK-123456".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "John Doe".to_string(),
            ticket_type: "VIP Admission".to_string(),
            verified_at: "10:30 AM".to_string(),
            status: "Valid".to_string(),
        },
        VerifiedTicket {
            id: "2".to_string(),
            ticket_code: "TIK-789012".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Jane Smith".to_string(),
            ticket_type: "Regular Admission".to_string(),
            verified_at: "10:45 AM".to_string(),
            status: "Valid".to_string(),
        },
        VerifiedTicket {
            id: "3".to_string(),
            ticket_code: "TIK-345678".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Mike Johnson".to_string(),
            ticket_type: "Early Bird".to_string(),
            verified_at: "11:00 AM".to_string(),
            status: "Valid".to_string(),
        },
        VerifiedTicket {
            id: "4".to_string(),
            ticket_code: "TIK-901234".to_string(),
            event_name: "Art Exhibition: Contemporary African Art".to_string(),
            attendee_name: "Sarah Williams".to_string(),
            ticket_type: "VIP Admission".to_string(),
            verified_at: "11:15 AM".to_string(),
            status: "Invalid".to_string(),
        },
    ];

    view! {
        <div class="verified-list-container">
            <div class="verified-header">
                <button class="btn-back" on:click=move |_| on_back.call(())>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="m15 18-6-6 6-6"/>
                    </svg>
                    "Back to Scanner"
                </button>
                <h2>"Verified Tickets"</h2>
            </div>

            <div class="verified-list">
                {tickets.into_iter().map(|ticket| {
                    let id = ticket.id.clone();
                    let ticket_clone = ticket.clone();
                    let on_select = on_select_ticket.clone();
                    
                    view! {
                        <div class="verified-card" on:click=move |_| on_select(id.clone())>
                            <div class="verified-card-status" data-status=ticket_clone.status.to_lowercase()>
                                {if ticket_clone.status == "Valid" {
                                    view! { <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg> }
                                } else {
                                    view! { <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg> }
                                }}
                            </div>
                            <div class="verified-card-content">
                                <h3 class="verified-name">{ticket_clone.attendee_name}</h3>
                                <div class="verified-meta">
                                    <span class="verified-code">{ticket_clone.ticket_code}</span>
                                    <span class="verified-time">{ticket_clone.verified_at}</span>
                                </div>
                                <div class="verified-type">{ticket_clone.ticket_type}</div>
                            </div>
                            <div class="verified-arrow">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="m9 18 6-6-6-6"/>
                                </svg>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
