use crate::services::skill_service::SkillService;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::grpc::skill_volunteering::Skill;

pub enum Msg {
    FetchSkills,
    SkillsFetched(Result<Vec<Skill>, String>),
    AddSkill(String),
    SkillAdded(Result<JsValue, JsValue>),
}

pub struct SkillList {
    skills: Vec<Skill>,
    error: Option<String>,
    skill_service: SkillService,
}

impl Component for SkillList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchSkills);
        Self {
            skills: vec![],
            error: None,
            skill_service: SkillService::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchSkills => {
                let link = ctx.link().clone();
                let mut skill_service = self.skill_service.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = skill_service.list_skills(100, 0, None).await;
                    let skills_result = result
                        .map(|js_value| serde_wasm_bindgen::from_value(js_value).unwrap())
                        .map_err(|e| e.to_string());
                    link.send_message(Msg::SkillsFetched(skills_result));
                });
                false
            }
            Msg::SkillsFetched(Ok(skills)) => {
                self.skills = skills;
                self.error = None;
                true
            }
            Msg::SkillsFetched(Err(e)) => {
                self.error = Some(e);
                true
            }
            Msg::AddSkill(skill_id) => {
                let link = ctx.link().clone();
                let mut skill_service = self.skill_service.clone();
                // Hardcoded user_id for now
                let user_id = "1".to_string();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = skill_service
                        .add_user_skill(user_id, skill_id, "beginner".to_string())
                        .await;
                    link.send_message(Msg::SkillAdded(result));
                });
                false
            }
            Msg::SkillAdded(Ok(_)) => {
                // We could refresh the user skill list here, but for now we do nothing.
                false
            }
            Msg::SkillAdded(Err(e)) => {
                self.error = Some(e.as_string().unwrap_or_else(|| "An unknown error occurred".to_string()));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{ "Available Skills" }</h2>
                {
                    if let Some(error) = &self.error {
                        html! { <p class="error">{ error }</p> }
                    } else {
                        html! {}
                    }
                }
                <ul>
                    { for self.skills.iter().map(|skill| self.view_skill(ctx, skill)) }
                </ul>
            </div>
        }
    }
}

impl SkillList {
    fn view_skill(&self, ctx: &Context<Self>, skill: &Skill) -> Html {
        let skill_id = skill.id.clone();
        let onclick = ctx.link().callback(move |_| Msg::AddSkill(skill_id.clone()));
        html! {
            <li>
                { &skill.name } ({ &skill.category })
                <button {onclick}>{ "Add" }</button>
            </li>
        }
    }
}

// Need to implement Clone for SkillService to use it in the async block
impl Clone for SkillService {
    fn clone(&self) -> Self {
        Self::new()
    }
}