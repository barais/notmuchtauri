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

export interface AppConfig {
  root_mail_dir: string;
  default_path: string;
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
}