export interface Message {
  id: string;
  subject: string;
  from: string;
  to: string;
  date: string;
  body: string;
  tags: string[];
  is_read: boolean;
  has_attachments: boolean;
}

export interface FolderNode {
  name: string;
  path: string;
  is_maildir: boolean;
  children: FolderNode[];
}

export interface AccountConfig{
  id:string,
  label: string,
  email: string,
  sent_folder?:string
}

export interface AppConfig {
  root_mail_dir: string;
  default_path: string;
  limit:number,
  accounts: AccountConfig[]
  default_sent_folder:string,
  rmtmmail?: string
  lthostport?: string
  calendaremail:string
  
}

export interface Thread {
  id: string;
  messages: string[];
  subject: string;
  last_message_date: string;
  participant_count: number;
}

export interface Attachment {
  filename: string;
  content_type: string;
  size: number;
  path: string;
}

export interface Draft {
  id: string;
  to: string;
  subject: string;
  body: string;
  attachments: Attachment[];
}


export interface AttachmentDto {
  partId: number;
  filename: string;
  contentType: string;
  contentId: string | null;
}

export interface MessageDto {
  id: string;
  subject: string;
  from: string;
  to: string;
  cc: string;
  date: string;
  timestamp: number;
  htmlBody: string | null;
  textBody: string | null;
  inlineImages: AttachmentDto[];
  attachments: AttachmentDto[];
  replies: MessageDto[];
    tags: string[];

}

export interface AddressMatch {
     name: string,
     email: string,
}


export interface ThreadDto {
  roots: MessageDto[];
}

export interface AttachmentPayload {
  path: string;
  mimeType?: string;
}

export interface EmailPayload {
  from: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  body: string;
  isHtml: boolean;
  attachments: AttachmentPayload[];
  account?: string;
  sentFolder:string
}

// --- Interfaces pour l'API LanguageTool ---
export  interface LTReplacement {
  value: string;
}

export interface LTContext {
  text: string;
  offset: number;
  length: number;
}

export interface ErrorOverlay {
  match: LTMatch;
  bounds: { top: number; left: number; width: number; height: number };
}


export interface LTMatch {
  message: string;
  shortMessage: string;
  replacements: LTReplacement[];
  offset: number;
  length: number;
  context: LTContext;
  rule: {
    id: string;
    description: string;
  };
}