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
