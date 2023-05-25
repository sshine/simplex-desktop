trait IChatCommand {
    fn ChatCommandTag(&self) -> String;
}

pub trait ShowActiveUser: IChatCommand {
    fn ShowActiveUserTag(&self) -> String;
}

pub trait CreateActiveUser: IChatCommand {
    fn CreateActiveUserTag(&self) -> String;
}

pub trait ListUsers: IChatCommand {
    fn ListUsersTag(&self) -> String;
}

pub trait APISetActiveUser: IChatCommand {
    fn APISetActiveUserTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn ViewPwd(&self) -> String;
}

pub trait APIHideUser: IChatCommand {
    fn APIHideUserTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn ViewPwd(&self) -> String;
}

pub trait APIUnhideUser: IChatCommand {
    fn APIUnhideUserTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn ViewPwd(&self) -> String;
}

pub trait APIMuteUser: IChatCommand {
    fn APIMuteUserTag(&self) -> String;
    fn UserId(&self) -> f32;
}

pub trait APIUnmuteUser: IChatCommand {
    fn APIUnmuteUserTag(&self) -> String;
    fn UserId(&self) -> f32;
}

pub trait APIDeleteUser: IChatCommand {
    fn APIDeleteUserTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn delSMPQues(&self) -> bool;
    fn ViewPwd(&self) -> String;
}

pub trait StartChat: IChatCommand {
    fn StartChatTag(&self) -> String;
    fn SubscribeConnections(&self) -> bool;
    fn EnableExpireChatOptions(&self) -> bool;
    fn StartXFTPWorkers(&self) -> bool;
}

pub trait APIStopChat: IChatCommand {
    fn APIStopChatTag(&self) -> String;
}

pub trait SetTempFolder: IChatCommand {
    fn SetTempFolderTag(&self) -> String;
    fn TempFolder(&self) -> String;
}

pub trait APISetXFTPConfig: IChatCommand {
    fn APISetXFTPConfigTag(&self) -> String;
    fn XFTPConfig(&self) -> XFTPFileConfig;
}

pub trait XFTPFileConfig {
    fn minFileSize(&self) -> f32;
}

pub trait SetIncognito: IChatCommand {
    fn SetIncognitoTag(&self) -> String;
    fn Incognito(&self) -> bool;
}

pub trait APIExportArchive: IChatCommand {
    fn APIExportArchiveTag(&self) -> String;
    fn ExportArchive(&self) -> bool;
}