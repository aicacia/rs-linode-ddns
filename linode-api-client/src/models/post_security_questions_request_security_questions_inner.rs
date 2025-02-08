/*
 * Akamai: Linode API
 *
 * Add a Cloud Computing instance so you can build, release, and scale applications faster with virtual machines. 
 *
 * The version of the OpenAPI document: 4.193.0
 * Contact: jperez@linode.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PostSecurityQuestionsRequestSecurityQuestionsInner : Single security question and response object for POST operation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityQuestionsRequestSecurityQuestionsInner {
    /// The ID representing the security question.
    #[serde(rename = "question_id", skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i32>,
    /// The security question response.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// __Read-only__ The security question.
    #[serde(rename = "security_question", skip_serializing_if = "Option::is_none")]
    pub security_question: Option<String>,
}

impl PostSecurityQuestionsRequestSecurityQuestionsInner {
    /// Single security question and response object for POST operation.
    pub fn new() -> PostSecurityQuestionsRequestSecurityQuestionsInner {
        PostSecurityQuestionsRequestSecurityQuestionsInner {
            question_id: None,
            response: None,
            security_question: None,
        }
    }
}

