use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntryInfo {
    title: String,
    date: String,
    tags: Vec<String>,
    path: Vec<String>,
    pdf: String,
    // mind rename from type to type
    r#type: String,
    file_type: String,
    uploader: String,
    hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntryInfoList {
    pub entries: Vec<EntryInfo>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    title: String,
    user: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>
}

#[test]
fn test_x() {
    let x = r#"[{"title": "Mathe H\u00dc", "date": "08.09.2022", "tags": ["Mathe", "Ralf_Eder", "MatheIstMacht", "08.09.2022", "pdf", "IT"], "path": ["6a49beff73e431d882fefa0e_0.jpeg", "6a49beff73e431d882fefa0e_1.jpeg"], "pdf": "6a49beff73e431d882fefa0e.pdf", "type": "other", "file_type": "pdf", "uploader": "admin", "hash": "6a49beff73e431d882fefa0e"}, {"title": "Mathe H\u00dc", "date": "08.09.2022", "tags": ["Mathe", "Ralf_Eder", "MatheIstMacht", "08.09.2022", "pdf", "IT"], "path": ["254ca8776414866634869dff_0.jpeg", "254ca8776414866634869dff_1.jpeg"], "pdf": "254ca8776414866634869dff.pdf", "type": "other", "file_type": "pdf", "uploader": "admin", "hash": "254ca8776414866634869dff"}, {"title": "Netzwerkkosten", "date": "07.09.2022", "tags": ["1BHITS", "Fischer", "08.09.2022", "pdf", "IT"], "path": ["pdf_logo/pdf.png"], "pdf": "b4db4dba26d0db967369672d.pdf", "type": "pdf", "file_type": "pdf", "uploader": "admin", "hash": "b4db4dba26d0db967369672d"}]"#;
    let a = r#"{"title": "Mathe H\u00dc", "date": "08.09.2022", "tags": ["Mathe", "Ralf_Eder", "MatheIstMacht", "08.09.2022", "pdf", "IT"], "path": ["6a49beff73e431d882fefa0e_0.jpeg", "6a49beff73e431d882fefa0e_1.jpeg"], "pdf": "6a49beff73e431d882fefa0e.pdf", "type": "other", "file_type": "pdf", "uploader": "admin", "hash": "6a49beff73e431d882fefa0e"}"#;
    let _entry: Result<EntryInfo, serde_json::Error> = serde_json::from_str(a); 
    //println!("entry: {entry:?}");

    let entries: Result<Vec<EntryInfo>, serde_json::Error> = serde_json::from_str(x); 
    println!("entries: {entries:?}");

    let f = r#"{"title": "Test", "user": "Stefan"}"#;

    let _entry: Result<User, serde_json::Error> = serde_json::from_str(f); 
    //println!("entry: {entry:?}");

    let users = r#"[{"title": "Test", "user": "Stefan"}, {"title": "asfd", "user": "Ferris"}]"#;
    let _entry: Result<Vec<User>, serde_json::Error> = serde_json::from_str(users); 

    //println!("entry: {entry:?}");
}