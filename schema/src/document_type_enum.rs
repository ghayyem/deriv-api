
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// Document type
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum DocumentTypeEnum {
    Passport,
    NationalIdentityCard,
    IdentificationNumberDocument,
    ServiceIdCard,
    DrivingLicence,
    UtilityBill,
    Bankstatement,
    BankStatement,
    PowerOfAttorney,
    Amlglobalcheck,
    Docverification,
    Proofid,
    Driverslicense,
    Proofaddress,
    Other,
    VoterCard,
    StudentCard,
    NimcSlip,
    BirthCertificate,
    PanCard,
    TaxPhotoId,
    SelfieWithId,
    PoiOthers,
    InsuranceBill,
    TaxReceipt,
    PhoneBill,
    PoaOthers,
    ProofOfOwnership,
    TaxReturn,
    EmploymentContract,
    Brokerage_Statement,
    Payslip,
    EddOthers,
    Coi,
    BusinessPoa,
    ArticleOfAssociation,
    Memorandum,
    AuthorisationLetter,
    Declarations,
    Affidavit,
    OfficialLetter,
    RentalAgreement,
    BusinessDocumentsOthers,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for DocumentTypeEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::Passport
    }
}
*/

