use yew::prelude::*;
use yew::platform::spawn_local;
use uuid::Uuid;
use crate::types::{CertificationData, CertificationType, SkillLevel};
use crate::services::grpc_client::SkillDevelopmentClient;

#[function_component(CertificationDisplay)]
pub fn certification_display() -> Html {
    let certifications = use_state(|| Vec::<CertificationData>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let certifications = certifications.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            let certifications = certifications.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                // In a real app, we would get the current user ID from context
                let user_id = Uuid::new_v4();
                
                // Create gRPC client
                let mut client = match SkillDevelopmentClient::new("http://localhost:50051".to_string()).await {
                    Ok(client) => client,
                    Err(e) => {
                        error.set(Some(format!("Failed to connect to server: {}", e)));
                        loading.set(false);
                        return;
                    }
                };

                // Call gRPC service
                match client.get_user_certifications(user_id).await {
                    Ok(cert_data) => {
                        certifications.set(cert_data);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch certifications: {}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }

    let on_verify = {
        Callback::from(move |cert: CertificationData| {
            // In a real app, this would verify the certification with the backend
            web_sys::console::log_1(&format!("Verifying certification: {}", cert.verification_code).into());
        })
    };

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Certifications"}</h2>
                <button class="btn btn-success">{"Share All"}</button>
            </div>
            if *loading {
                <div class="card-body">
                    <div class="text-center">
                        <div class="spinner-border" role="status">
                            <span class="sr-only">{"Loading..."}</span>
                        </div>
                    </div>
                </div>
            } else if let Some(err) = &*error {
                <div class="card-body">
                    <div class="alert alert-danger">
                        {err}
                    </div>
                </div>
            } else {
                <div class="grid grid-cols-3">
                    {for certifications.iter().map(|cert| {
                        let on_verify = on_verify.clone();
                        let cert = cert.clone();
                        html! {
                            <div class="card">
                                <h3>{&cert.skill_name}</h3>
                                <div class="grid">
                                    <span>{format!("{:?}", cert.level_achieved)}</span>
                                    <span>{format!("{:?}", cert.certification_type)}</span>
                                    <span>{&cert.issued_at}</span>
                                </div>
                                <button
                                    class="btn btn-secondary"
                                    onclick={move |_| on_verify.emit(cert.clone())}
                                >
                                    {"Verify"}
                                </button>
                           </div>
                       }
                    })}
                </div>
            }
        </div>
    }
}