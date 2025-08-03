//! Tests for the survey module
//! 
//! This module contains tests for all the functionality in the survey system.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use crate::analysis::*;
    use crate::visualization::*;
    use crate::template_service::*;
    use crate::error::*;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_create_survey() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
                Question::TextResponse {
                    max_length: Some(100),
                },
            ],
            scoring_config: None,
        };
        
        assert_eq!(survey.title, "Test Survey");
        assert_eq!(survey.questions.len(), 2);
    }
    
    #[test]
    fn test_create_survey_response() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
                Question::TextResponse {
                    max_length: Some(100),
                },
            ],
            scoring_config: None,
        };
        
        let response = SurveyResponse {
            survey_id: survey.id,
            answers: vec![
                Answer::StarRating(4.5),
                Answer::TextResponse("Great survey!".to_string()),
            ],
            created_at: Utc::now(),
        };
        
        assert_eq!(response.survey_id, survey.id);
        assert_eq!(response.answers.len(), 2);
    }
    
    #[test]
    fn test_calculate_average_star_rating() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
            ],
            scoring_config: None,
        };
        
        let responses = vec![
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(4.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(3.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(5.0)],
                created_at: Utc::now(),
            },
        ];
        
        let avg = calculate_average_star_rating(&responses, 0).unwrap();
        assert_eq!(avg, 4.3333335); // (4.5 + 3.5 + 5.0) / 3
    }
    
    #[test]
    fn test_create_template() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
                Question::TextResponse {
                    max_length: Some(100),
                },
            ],
            scoring_config: None,
        };
        
        let template = create_template(survey);
        assert_eq!(template.questions.len(), 2);
        assert_eq!(template.version, 1);
    }
    
    #[test]
    fn test_apply_template() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
                Question::TextResponse {
                    max_length: Some(100),
                },
            ],
            scoring_config: None,
        };
        
        let template = create_template(survey);
        let new_survey = apply_template(template);
        assert_eq!(new_survey.questions.len(), 2);
    }
    
    #[test]
    fn test_calculate_correlation() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
            ],
            scoring_config: None,
        };
        
        let responses = vec![
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(4.5), Answer::StarRating(4.0)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(3.5), Answer::StarRating(3.0)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(5.0), Answer::StarRating(4.5)],
                created_at: Utc::now(),
            },
        ];
        
        let correlation = calculate_correlation(&responses, 0, 1).unwrap().unwrap();
        assert!(correlation > 0.0); // Should be positive correlation
    }
    
    #[test]
    fn test_analyze_trends() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
            ],
            scoring_config: None,
        };
        
        let responses = vec![
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(4.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(3.5)],
                created_at: Utc::now(),
            },
        ];
        
        let trends = analyze_trends(&responses, 0, TimePeriod::Daily).unwrap();
        assert!(!trends.periods.is_empty());
    }
    
    #[test]
    fn test_sampled_responses() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
            ],
            scoring_config: None,
        };
        
        let responses = vec![
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(4.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(3.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(5.0)],
                created_at: Utc::now(),
            },
        ];
        
        let sampled = sampled_responses(&responses, 2).unwrap();
        assert_eq!(sampled.len(), 2);
    }
    
    #[test]
    fn test_analyze_sentiment() {
        let texts = vec![
            "This is a great product!".to_string(),
            "This is a terrible product.".to_string(),
            "This product is okay.".to_string(),
        ];
        
        let sentiments = analyze_sentiment(&texts);
        assert_eq!(sentiments.len(), 3);
    }
    
    #[test]
    fn test_compare_demographic_groups() {
        let survey = Survey {
            id: Uuid::new_v4(),
            title: "Test Survey".to_string(),
            description: "A test survey".to_string(),
            questions: vec![
                Question::StarRating {
                    min: 0.0,
                    max: 5.0,
                    step: 0.5,
                },
            ],
            scoring_config: None,
        };
        
        let responses = vec![
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(4.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(3.5)],
                created_at: Utc::now(),
            },
            SurveyResponse {
                survey_id: survey.id,
                answers: vec![Answer::StarRating(5.0)],
                created_at: Utc::now(),
            },
        ];
        
        let comparison = compare_demographic_groups(&responses, DemographicField::Age, 0).unwrap();
        assert!(!comparison.is_empty());
    }
    
    #[test]
    fn test_visualization_histogram() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = visualization::plot::histogram(&data, 5);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_visualization_heatmap() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let result = visualization::plot::heatmap(&matrix);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_visualization_word_cloud() {
        let words = vec![("good", 0.8), ("bad", 0.2), ("excellent", 0.9)];
        let result = visualization::plot::word_cloud(&words);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_visualization_trend_line() {
        let points = vec![
            (Utc::now(), 1.0),
            (Utc::now(), 2.0),
            (Utc::now(), 3.0),
        ];
        let result = visualization::plot::trend_line(&points);
        assert!(result.is_ok());
    }
    
}