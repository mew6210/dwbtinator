use winapi::shared::windef::HICON;

pub struct Process{
    pub title_name:String,
    pub exec_name:String,
    pub icon:Option<HICON>
}