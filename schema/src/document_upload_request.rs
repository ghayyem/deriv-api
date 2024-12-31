
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Request KYC information from client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DocumentUploadRequest {
    /// Document file format
    #[serde(rename = "document_format")]
    pub document_format: DocumentFormatEnum,
    /// [Optional] Document ID (required for Passport, Proof of ID and Driver's License)
    #[serde(rename = "document_id", skip_serializing_if = "Option::is_none")]
    pub document_id: String,
    /// 2-letter country code, mandatory for POI only
    #[serde(rename = "document_issuing_country", skip_serializing_if = "Option::is_none")]
    pub document_issuing_country: String,
    /// Document type
    #[serde(rename = "document_type")]
    pub document_type: DocumentTypeEnum,
    /// Must be `1`
    #[serde(rename = "document_upload")]
    pub document_upload: DocumentUploadEnum,
    /// The checksum of the file to be uploaded
    #[serde(rename = "expected_checksum")]
    pub expected_checksum: String,
    /// [Optional] Document expiration date (required for Passport, Proof of ID and Driver's License)
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: String,
    /// Document size (should be less than 10MB)
    #[serde(rename = "file_size")]
    pub file_size: i64,
    /// [Optional] Boolean value that indicates whether this document is lifetime valid (only applies to POI document types, cancels out the expiration_date given if any)
    #[serde(rename = "lifetime_valid", skip_serializing_if = "Option::is_none")]
    pub lifetime_valid: LifetimeValidEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] To determine document side
    #[serde(rename = "page_type", skip_serializing_if = "Option::is_none")]
    pub page_type: PageTypeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] It contains info about the proof of ownership being uploaded (mandatory for proof_of_ownership document type)
    #[serde(rename = "proof_of_ownership", skip_serializing_if = "Option::is_none")]
    pub proof_of_ownership: ProofOfOwnership,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// [Optional] It contains info about the proof of ownership being uploaded (mandatory for proof_of_ownership document type)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProofOfOwnership {
    /// A collection of unspecific information related to the proof of ownership being uploaded
    #[serde(rename = "details")]
    pub details: Details,
    /// The id of the proof of ownership as shown in the /get_account_status proof of ownership list
    #[serde(rename = "id")]
    pub id: f64,
}






/// Document file format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFormatEnum {
    PNG,
    JPG,
    JPEG,
    GIF,
    PDF,
}

impl DocumentFormatEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PNG => "PNG",
            Self::JPG => "JPG",
            Self::JPEG => "JPEG",
            Self::GIF => "GIF",
            Self::PDF => "PDF",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PNG" => Some(Self::PNG),
            "JPG" => Some(Self::JPG),
            "JPEG" => Some(Self::JPEG),
            "GIF" => Some(Self::GIF),
            "PDF" => Some(Self::PDF),
            _ => None,
        }
    }
}
/// Document type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentTypeEnum {
    Passport,
    National_Identity_Card,
    Identification_Number_Document,
    Service_Id_Card,
    Driving_Licence,
    Utility_Bill,
    Bankstatement,
    Bank_Statement,
    Power_Of_Attorney,
    Amlglobalcheck,
    Docverification,
    Proofid,
    Driverslicense,
    Proofaddress,
    Other,
    Voter_Card,
    Student_Card,
    Nimc_Slip,
    Birth_Certificate,
    Pan_Card,
    Tax_Photo_Id,
    Selfie_With_Id,
    Poi_Others,
    Insurance_Bill,
    Tax_Receipt,
    Phone_Bill,
    Poa_Others,
    Proof_Of_Ownership,
    Tax_Return,
    Employment_Contract,
    Brokerage_Statement,
    Payslip,
    Edd_Others,
    Coi,
    Business_Poa,
    Article_Of_Association,
    Memorandum,
    Authorisation_Letter,
    Declarations,
    Affidavit,
    Official_Letter,
    Rental_Agreement,
    Business_Documents_Others,
}

impl DocumentTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passport => "passport",
            Self::National_Identity_Card => "national_identity_card",
            Self::Identification_Number_Document => "identification_number_document",
            Self::Service_Id_Card => "service_id_card",
            Self::Driving_Licence => "driving_licence",
            Self::Utility_Bill => "utility_bill",
            Self::Bankstatement => "bankstatement",
            Self::Bank_Statement => "bank_statement",
            Self::Power_Of_Attorney => "power_of_attorney",
            Self::Amlglobalcheck => "amlglobalcheck",
            Self::Docverification => "docverification",
            Self::Proofid => "proofid",
            Self::Driverslicense => "driverslicense",
            Self::Proofaddress => "proofaddress",
            Self::Other => "other",
            Self::Voter_Card => "voter_card",
            Self::Student_Card => "student_card",
            Self::Nimc_Slip => "nimc_slip",
            Self::Birth_Certificate => "birth_certificate",
            Self::Pan_Card => "pan_card",
            Self::Tax_Photo_Id => "tax_photo_id",
            Self::Selfie_With_Id => "selfie_with_id",
            Self::Poi_Others => "poi_others",
            Self::Insurance_Bill => "insurance_bill",
            Self::Tax_Receipt => "tax_receipt",
            Self::Phone_Bill => "phone_bill",
            Self::Poa_Others => "poa_others",
            Self::Proof_Of_Ownership => "proof_of_ownership",
            Self::Tax_Return => "tax_return",
            Self::Employment_Contract => "employment_contract",
            Self::Brokerage_Statement => "brokerage statement",
            Self::Payslip => "payslip",
            Self::Edd_Others => "edd_others",
            Self::Coi => "coi",
            Self::Business_Poa => "business_poa",
            Self::Article_Of_Association => "article_of_association",
            Self::Memorandum => "memorandum",
            Self::Authorisation_Letter => "authorisation_letter",
            Self::Declarations => "declarations",
            Self::Affidavit => "affidavit",
            Self::Official_Letter => "official_letter",
            Self::Rental_Agreement => "rental_agreement",
            Self::Business_Documents_Others => "business_documents_others",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "passport" => Some(Self::Passport),
            "national_identity_card" => Some(Self::National_Identity_Card),
            "identification_number_document" => Some(Self::Identification_Number_Document),
            "service_id_card" => Some(Self::Service_Id_Card),
            "driving_licence" => Some(Self::Driving_Licence),
            "utility_bill" => Some(Self::Utility_Bill),
            "bankstatement" => Some(Self::Bankstatement),
            "bank_statement" => Some(Self::Bank_Statement),
            "power_of_attorney" => Some(Self::Power_Of_Attorney),
            "amlglobalcheck" => Some(Self::Amlglobalcheck),
            "docverification" => Some(Self::Docverification),
            "proofid" => Some(Self::Proofid),
            "driverslicense" => Some(Self::Driverslicense),
            "proofaddress" => Some(Self::Proofaddress),
            "other" => Some(Self::Other),
            "voter_card" => Some(Self::Voter_Card),
            "student_card" => Some(Self::Student_Card),
            "nimc_slip" => Some(Self::Nimc_Slip),
            "birth_certificate" => Some(Self::Birth_Certificate),
            "pan_card" => Some(Self::Pan_Card),
            "tax_photo_id" => Some(Self::Tax_Photo_Id),
            "selfie_with_id" => Some(Self::Selfie_With_Id),
            "poi_others" => Some(Self::Poi_Others),
            "insurance_bill" => Some(Self::Insurance_Bill),
            "tax_receipt" => Some(Self::Tax_Receipt),
            "phone_bill" => Some(Self::Phone_Bill),
            "poa_others" => Some(Self::Poa_Others),
            "proof_of_ownership" => Some(Self::Proof_Of_Ownership),
            "tax_return" => Some(Self::Tax_Return),
            "employment_contract" => Some(Self::Employment_Contract),
            "brokerage statement" => Some(Self::Brokerage_Statement),
            "payslip" => Some(Self::Payslip),
            "edd_others" => Some(Self::Edd_Others),
            "coi" => Some(Self::Coi),
            "business_poa" => Some(Self::Business_Poa),
            "article_of_association" => Some(Self::Article_Of_Association),
            "memorandum" => Some(Self::Memorandum),
            "authorisation_letter" => Some(Self::Authorisation_Letter),
            "declarations" => Some(Self::Declarations),
            "affidavit" => Some(Self::Affidavit),
            "official_letter" => Some(Self::Official_Letter),
            "rental_agreement" => Some(Self::Rental_Agreement),
            "business_documents_others" => Some(Self::Business_Documents_Others),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentUploadEnum {
    Value1 = 1,
}

impl DocumentUploadEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// [Optional] Boolean value that indicates whether this document is lifetime valid (only applies to POI document types, cancels out the expiration_date given if any)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifetimeValidEnum {
    Value0,
    Value1 = 1,
}

impl LifetimeValidEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// [Optional] To determine document side
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageTypeEnum {
    Front,
    Back,
    Photo,
}

impl PageTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Front => "front",
            Self::Back => "back",
            Self::Photo => "photo",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "front" => Some(Self::Front),
            "back" => Some(Self::Back),
            "photo" => Some(Self::Photo),
            _ => None,
        }
    }
}
