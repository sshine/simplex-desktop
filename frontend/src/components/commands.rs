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
    fn ExportArchive(&self) -> ArchiveConfig;
}

pub trait APIImportArchive: IChatCommand {
    fn APIImportArchiveTag(&self) -> String;
    fn ImportArchive(&self) -> ArchiveConfig;
}

pub trait APIDeleteStorage: IChatCommand {
    fn APIDeleteStorageTag(&self) -> String;
}

pub trait APIGetChats: IChatCommand {
    fn APIGetChatsTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn PendingConnection(&self) -> bool;
}

pub trait APIGetChat: IChatCommand {
    fn APIGetChatTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
    fn Pagnation(&self) -> ChatPagination;
    fn Search(&self) -> String;
}

pub trait APISendMessage: IChatCommand {
    fn APISendMessageTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
    fn Message(&self) -> ComposedMessage;
}

pub trait ComposedMessage {
    fn FilePath(&self) -> String;
    fn QuotedItemId(&self) -> ChatItemId;
    fn MsgContent(&self) -> String;
}

pub trait APIUpdatechatItem: IChatCommand {
    fn APIUpdatechatItemTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
    fn ChatItemId(&self) -> ChatItemId;
    fn MsgContent(&self) -> MsgContent;
}

pub trait APIDeleteChatItem: IChatCommand {
    fn APIDeleteChatItemTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
    fn ChatItemId(&self) -> ChatItemId;
    fn DeleteMode(&self) -> DeleteMode;
}

pub trait APIDeleteMemberChatItem: IChatCommand {
    fn APIDeleteMemberChatItemTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn GroupMemberId(&self) -> f32;
    fn ItemId(&self) -> f32;
}

pub trait APIChatRead: IChatCommand {
    fn APIChatReadTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
    fn ItemRange(&self) -> ItemRange;
}

pub trait ItemRange {
    fn FromItem(&self) -> ChatItemId;
    fn ToItem(&self) -> ChatItemId;
}

pub trait APIDeleteChat: IChatCommand {
    fn APIDeleteChatTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
}

pub trait APIClearChat: IChatCommand {
    fn APIClearChatTag(&self) -> String;
    fn ChatType(&self) -> ChatType;
    fn ChatId(&self) -> f32;
}

pub trait APIAcceptContact: IChatCommand {
    fn APIAcceptContactTag(&self) -> String;
    fn ContactId(&self) -> f32;
}

pub trait APIRejectContact: IChatCommand {
    fn APIRejectContactTag(&self) -> String;
    fn ContactId(&self) -> f32;
}

pub trait APIUpdateProfile: IChatCommand {
    fn APIUpdateProfileTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn Profile(&self) -> Profile;
}

pub trait APISetcontactAlias: IChatCommand {
    fn APISetcontactAliasTag(&self) -> String;
    fn ContactId(&self) -> f32;
    fn LocalAlias(&self) -> String;
}

pub trait APIParseMarkdown: IChatCommand {
    fn APIParseMarkdownTag(&self) -> String;
    fn Text(&self) -> String;
}

pub trait NewGroup: IChatCommand {
    fn NewGroupTag(&self) -> String;
    fn GroupProfile(&self) -> GroupProfile;
}

pub trait APIAddMember: IChatCommand {
    fn APIAddMemberTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn ContactId(&self) -> f32;
    fn MemberRole(&self) -> GroupMemberRole;
}

pub trait APIJoinGroup: IChatCommand {
    fn APIJoinGroupTag(&self) -> String;
    fn GroupId(&self) -> f32;
}

pub trait APIRemoveMember: IChatCommand {
    fn APIRemoveMemberTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn MemberId(&self) -> f32;
}

pub trait APILeaveGroup: IChatCommand {
    fn APILeaveGroupTag(&self) -> String;
    fn GroupId(&self) -> f32;
}

pub trait APIListMembers: IChatCommand {
    fn APIListMembersTag(&self) -> String;
    fn GroupId(&self) -> f32;
}

pub trait APIUpdateGroupProfile: IChatCommand {
    fn APIUpdateGroupProfileTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn GroupProfile(&self) -> GroupProfile;
}

pub trait APICreateGroupLink: IChatCommand {
    fn APICreateGroupLinkTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn MemberRole(&self) -> GroupMemberRole;
}

pub trait APIGroupLinkMemberRole: IChatCommand {
    fn APIGroupLinkMemberRoleTag(&self) -> String;
    fn GroupId(&self) -> f32;
    fn MemberRole(&self) -> GroupMemberRole;
}

pub trait APIDeleteGroupLink: IChatCommand {
    fn APIDeleteGroupLinkTag(&self) -> String;
    fn GroupId(&self) -> f32;
}

pub trait APIGetGroupLink: IChatCommand {
    fn APIGetGroupLinkTag(&self) -> String;
    fn GroupId(&self) -> f32;
}

pub trait APIGetUserProtoServers: IChatCommand {
    fn APIGetUserProtoServersTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn ServerProtocol(&self) -> ServerProtocol;
}

pub trait APISetUserProtoServers: IChatCommand {
    fn APISetUserProtoServersTag(&self) -> String;
    fn UserId(&self) -> f32;
    fn ServerProtocol(&self) -> ServerProtocol;
    fn Servers(&self) -> Vec<ServerCfg>;
}

pub trait ServerCfg {
    fn Server(&self) -> String;
    fn Preset(&self) -> bool;
    fn Tested(&self) -> bool;
    fn Enabled(&self) -> bool;
}

let const SMP = "smp";
let const XFTP = "xftp";

pub enum ServerProtocol {
    SMP,
    XFTP
}

