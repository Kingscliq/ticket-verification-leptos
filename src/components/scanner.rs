use leptos::*;
use gloo_file::File;
use gloo_file::futures::read_as_bytes;
use crate::components::result::TicketStatus;
use crate::api::{TicketData, ItemDetails};


#[component]
fn TicketDetails(
    data: TicketData,
    #[prop(into)]
    on_reset: Callback<()>
) -> impl IntoView {
    view! {
         <div class="ticket-details-card">
             <div class="ticket-success-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/>
                    <path d="m9 12 2 2 4-4"/>
                </svg>
             </div>
             <h3 class="ticket-title">{data.item_title}</h3>
             
             <div class="ticket-info-list">
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Booking Reference:"</span>
                     <span class="ticket-detail-value">{data.booking_number}</span>
                 </div>
                 
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Ticket Code:"</span>
                     <span class="ticket-detail-value">{data.ticket_code}</span>
                 </div>
                 
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Item Type:"</span>
                     <span class="ticket-detail-value">{data.item_type}</span>
                 </div>
                 
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Quantity:"</span>
                     <span class="ticket-detail-value">{data.quantity}</span>
                 </div>
                 
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Date & Time:"</span>
                     <span class="ticket-detail-value">
                         {format!("{} • {}", data.item_details.date, data.item_details.time)}
                     </span>
                 </div>
                 
                 <div class="ticket-detail-row">
                     <span class="ticket-detail-label">"Location:"</span>
                     <span class="ticket-detail-value right-align">{data.item_details.location}</span>
                 </div>
             </div>

             <button class="btn-scan-next" on:click=move |_| {
                  on_reset.call(());
             }>"Scan Next"</button>
         </div>
    }
}

#[component]
pub fn Scanner<F>(
    #[prop(into)]
    on_scan: Callback<String>,
    #[prop(into)]
    on_reset: Callback<()>,
    #[prop(into)]
    on_back: Callback<()>,
    #[prop(into)]
    on_history: Callback<()>,
    status: F,
) -> impl IntoView 
where F: Fn() -> TicketStatus + 'static + Clone
{
    let (input_mode, set_input_mode) = create_signal("scan".to_string()); // "scan" or "manual"
    let (manual_code, set_manual_code) = create_signal(String::new());
    let (status_msg, set_status_msg) = create_signal("Ready".to_string());

    // Create a memo for the status to allow easy sharing across closures
    let status_memo = create_memo(move |_| status());

    create_effect(move |_| {
        let current_status = status_memo.get();
        if current_status != TicketStatus::Verifying && current_status != TicketStatus::Idle {
             set_status_msg.set("Scan Complete".to_string());
             set_manual_code.set("".to_string());
        }
    });

    let on_file_input = move |ev: ev::Event| {
        let input_el: web_sys::HtmlInputElement = event_target(&ev);
        if let Some(files) = input_el.files() {
             if let Some(file) = files.get(0) {
                 let filename = file.name();
                 
                 // Check file type
                 if filename.to_lowercase().ends_with(".svg") {
                     set_status_msg.set("SVG files not supported. Please use PNG or JPEG.".to_string());
                     return;
                 }
                 
                 set_status_msg.set("Processing image...".to_string());
                 
                 let file = File::from(file);
                 let set_status_msg_clone = set_status_msg.clone();
                 let status_memo_clone = status_memo.clone();
                 let set_manual_code_clone = set_manual_code.clone();
                                  
                 spawn_local(async move {
                     match read_as_bytes(&file).await {
                         Ok(bytes) => {
                             set_status_msg_clone.set("Decoding QR...".to_string());
                             match image::load_from_memory(&bytes) {
                                  Ok(img) => {
                                      let img = if img.width() > 800 || img.height() > 800 {
                                          img.resize(800, 800, image::imageops::FilterType::Triangle)
                                      } else { img };
                                      
                                      let img = img.to_luma8();
                                      let mut img = rqrr::PreparedImage::prepare(img);
                                      let grids = img.detect_grids();
                                      
                                      if let Some(grid) = grids.first() {
                                          match grid.decode() {
                                              Ok((_meta, content)) => {
                              if let TicketStatus::Valid(_) = status_memo_clone.get() {
                set_status_msg_clone.set("Scan Complete".to_string());
                set_manual_code_clone.set("".to_string());
            }
                      on_scan.call(content);
                                              }
                                              Err(e) => set_status_msg_clone.set(format!("Failed to decode QR: {:?}", e)),
                                          }
                                      } else {
                                          set_status_msg_clone.set("No QR code found".to_string());
                                      }
                                  },
                                  Err(e) => set_status_msg_clone.set(format!("Failed to load image: {:?}", e)),
                             }
                         },
                         Err(e) => set_status_msg_clone.set(format!("File read error: {:?}", e)),
                     }
                 });
             }
        }
    };

    view! {
        <div class="input-section card">
            <div class="scanner-header">
                <div class="scanner-actions">
                    <button class="btn-icon" on:click=move |_| on_back.call(()) title="Back to Events">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="m15 18-6-6 6-6"/>
                        </svg>
                    </button>
                    <button class="btn-text" on:click=move |_| on_history.call(())>
                        "History"
                    </button>
                </div>
                <h2>"Scan Ticket"</h2>
                <p>"Point your camera at the QR code"</p>
            </div>
            <div class="card-content">
                // Feedback Alerts
                {move || match status_memo.get() {
                    TicketStatus::Valid(_) => view! {
                        <div class="alert-success">
                            "Valid Ticket"
                        </div>
                    }.into_view(),
                    TicketStatus::Invalid => view! {
                        <div class="alert-error">
                            "Invalid Ticket"
                        </div>
                    }.into_view(),
                    TicketStatus::Used => view! {
                        <div class="alert-error">
                            "Already Used"
                        </div>
                    }.into_view(),
                    TicketStatus::Error(e) => view! {
                        <div class="alert-error">
                            {e}
                        </div>
                    }.into_view(),
                    _ => view! { <div class="hidden"></div> }.into_view()
                }}

                <div class="tabs">
                    <button 
                        class=move || if input_mode.get() == "scan" { "tab-btn active" } else { "tab-btn" }
                        on:click=move |_| set_input_mode.set("scan".to_string())
                    >
                        "Scan QR"
                    </button>
                    <button 
                        class=move || if input_mode.get() == "manual" { "tab-btn active" } else { "tab-btn" }
                        on:click=move |_| set_input_mode.set("manual".to_string())
                    >
                        "Enter Code"
                    </button>
                </div>

                {move || {
                    let global_status = status_memo.get();
                    // If we have a valid ticket, show the details card regardless of input input_mode
                    // UNLESS the user explicitly switches modes? No, if it's valid, we want to show it until reset.
                    // But the existing logic splits by mode. Let's keep the split but reuse component.
                    
                    if input_mode.get() == "scan" {
                        if let TicketStatus::Valid(data) = global_status.clone() {
                                view! { <TicketDetails data=data on_reset=on_reset /> }.into_view()
                        } else {
                            view! {
                                <div class="scanner-wrapper">
                                        <div class="scanner-overlay"></div>
                                        
                                        <div class="scanner-status-overlay">
                                            {move || {
                                                let current_msg = status_msg.get();
                                                if current_msg != "Ready" && current_msg != "Scan Complete" {
                                                    view! { <p class="scanner-status-text">{current_msg}</p> }.into_view()
                                                } else {
                                                    match global_status {
                                                        TicketStatus::Invalid => view! { <p class="scanner-status-text red">"Invalid Ticket"</p> }.into_view(),
                                                        TicketStatus::Used => view! { <p class="scanner-status-text yellow">"Already Used"</p> }.into_view(),
                                                        TicketStatus::Verifying => view! { <p class="scanner-status-text">"Verifying..."</p> }.into_view(),
                                                        _ => view! { <p class="scanner-hint-overlay">"Tap to Scan or Upload QR"</p> }.into_view(),
                                                    }
                                                }
                                            }}
                                        </div>
                                        
                                        <input type="file" 
                                            accept="image/png,image/jpeg,image/jpg,image/webp" 
                                            capture="environment" 
                                            class="scanner-file-input"
                                            on:change=on_file_input
                                        />
                                </div>
                            }.into_view()
                        }
                    } else {
                        // Manual Mode
                        if let TicketStatus::Valid(data) = global_status.clone() {
                                view! { <TicketDetails data=data on_reset=on_reset /> }.into_view()
                        } else {
                            view! {
                                    <div class="manual-input-container">
                                    <div class="input-group">
                                        <input 
                                            type="text" 
                                            class="input-field"
                                            placeholder="e.g. TICK-12345"
                                            prop:value=manual_code
                                            on:input=move |ev| set_manual_code.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <button 
                                        class="btn btn-primary w-full mt-4"
                                        disabled=move || manual_code.get().is_empty() || status_memo.get() == TicketStatus::Verifying
                                        on:click=move |_| {
                                            on_scan.call(manual_code.get());
                                        }
                                    >
                                        {move || if status_memo.get() == TicketStatus::Verifying {
                                            "Verifying..."
                                        } else {
                                            "Verify Ticket"
                                        }}
                                    </button>
                                        
                                    </div>
                            }.into_view()
                        }
                    }
                }}
            </div>
        </div>
    }
}
