use yew::prelude::*;
use std::collections::HashMap;
use crate::services::skill_development::{SkillProgress, Certification};

#[derive(Debug, Clone, PartialEq)]
pub struct SkillState {
    pub skills: Vec<SkillProgress>,
    pub certifications: Vec<Certification>,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SkillAction {
    SetSkills(Vec<SkillProgress>),
    SetCertifications(Vec<Certification>),
    AddSkill(SkillProgress),
    UpdateSkill(SkillProgress),
    SetLoading(bool),
    SetError(Option<String>),
}

impl Reducible for SkillState {
    type Action = SkillAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SkillAction::SetSkills(skills) => {
                Rc::new(SkillState {
                    skills,
                    certifications: self.certifications.clone(),
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            SkillAction::SetCertifications(certifications) => {
                Rc::new(SkillState {
                    skills: self.skills.clone(),
                    certifications,
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            SkillAction::AddSkill(skill) => {
                let mut skills = self.skills.clone();
                skills.push(skill);
                Rc::new(SkillState {
                    skills,
                    certifications: self.certifications.clone(),
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            SkillAction::UpdateSkill(updated_skill) => {
                let mut skills = self.skills.clone();
                if let Some(index) = skills.iter().position(|s| s.id == updated_skill.id) {
                    skills[index] = updated_skill;
                }
                Rc::new(SkillState {
                    skills,
                    certifications: self.certifications.clone(),
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            SkillAction::SetLoading(loading) => {
                Rc::new(SkillState {
                    skills: self.skills.clone(),
                    certifications: self.certifications.clone(),
                    loading,
                    error: self.error.clone(),
                })
            }
            SkillAction::SetError(error) => {
                Rc::new(SkillState {
                    skills: self.skills.clone(),
                    certifications: self.certifications.clone(),
                    loading: self.loading,
                    error,
                })
            }
        }
    }
}

#[function_component(SkillContextProvider)]
pub fn skill_context_provider(props: &ChildrenProperties) -> Html {
    let skill_state = use_reducer(|| SkillState {
        skills: Vec::new(),
        certifications: Vec::new(),
        loading: false,
        error: None,
    });

    html! {
        <ContextProvider<UseReducerHandle<SkillState>> context={skill_state}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<SkillState>>>
    }
}

pub fn use_skills() -> UseReducerHandle<SkillState> {
    use_context::<UseReducerHandle<SkillState>>()
        .expect("Skill context is missing")
}