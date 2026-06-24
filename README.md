# NotmuchTauri

**NotmuchTauri** is a fast, lightweight Mail User Agent (MUA) developed using **Tauri, Vue 3, TypeScript, and Tailwind CSS**, built on top of [Notmuch](https://notmuchmail.org/) for Linux and macOS environments. It provides a ligthweigth integration of Custom Providers (OpenAI LLM integration) 

🎥 **[Watch a short demo here](https://videos.diverse-team.fr/notmuch-electron.html)**

---

## 📖 Philosophy & Use Cases

Before getting started, it helps to understand the email workflow this application is designed for. NotmuchTauri is built for power users who share the following needs:

- **Search over sorting**: I don't waste time sorting emails into complex project or customer folders. I prefer to archive everything and rely on blazing-fast searches.
- **Multiple Accounts**: I manage several IMAP email accounts from different service providers.
- **Local Backup & Offline Mode**: All emails are synced to a local folder on my computer. I travel a lot, and I need a mail client that works flawlessly without an internet connection or during network drops.
- **Push Notifications**: I do not want to fetch my emails manually; I rely on automated background synchronization.
- **Plain Text & HTML**: I need to seamlessly read and write both text and HTML emails in multiple languages (English and French).
- **Productivity Integration**: I want to quickly create meetings or tasks (e.g., Remember The Milk) directly from my emails.

To achieve this, NotmuchTauri acts as a modern GUI coordinator for the best CLI email tools available in the Unix ecosystem:
*   **[Notmuch](https://notmuchmail.org/)**: The core mail indexer and search engine.
*   **[mbsync (isync)](http://isync.sourceforge.net/)**: To synchronize IMAP mailboxes locally.
*   **[msmtp](https://marlam.de/msmtp/)**: To send emails via SMTP.
*   **[afew](https://github.com/afewmail/afew)**: To automatically tag incoming emails.

---

## ✨ Features

- [x] Read emails from several IMAP services (via `mbsync`)
- [x] Compose new emails (HTML / Text)
- [x] Reply and Forward emails
- [x] Fast email querying and indexing (via `notmuch`)
- [x] SMTP server selection per account for sending emails
- [x] Mail address completion based on mail history
- [ ] LLM integration Custom Providers (OpenAI/Anthropic Compatible)
- [ ] Edit as new email
- [ ] Notmuch query auto-completion
- [ ] ICS event creation from email
- [ ] [Remember The Milk](https://www.rememberthemilk.com/) task creation from email
- [ ] Email spell check with language detection (using `Ctrl + Tab`)
- [ ] Customizable keyboard shortcuts
- [ ] Customizable text snippets (useful for greetings)
- [ ] Outbox folder management (send emails when network is back up)
- [ ] Save emails as drafts
- [ ] IMAP IDLE support for instant push notifications
- [ ] LDAP integration for address completion


---

## 🚀 Installation & Prerequisites

To use NotmuchTauri, you first need to install and configure the underlying CLI tools.

### 1. Install CLI Tools
*Requirement: Notmuch >= 0.26, afew >= 1.3*

On Debian/Ubuntu:
```bash
sudo apt-get install msmtp isync notmuch notmuch-addrlookup afew gnupg2 ca-certificates parallel mktemp
```

---

## ⚙️ cli tools (mbsync, afew, notmuch, msmtp) Configuration

### 🛠 Backend Tools Configuration Guide

*Note: Replace `/home/barais` and specific email addresses with your own in the examples below.*

### Configure `mbsync`
Create folders for your mail providers:

```bash
mkdir -p ~/mail/IRISA ~/mail/gmail
```

Securely store your passwords using `gpg2` (use your password manager to retrieve them):

```bash
echo -e "your_password" | gpg2 --symmetric -o ~/mail/zimbra.inria.fr.gpg
```

Create a `~/.mbsyncrc` file:

```text
IMAPAccount  main
Host zimbra.inria.fr
User barais
PassCmd "gpg2 -q --for-your-eyes-only --no-tty -d ~/mail/zimbra.inria.fr.gpg"
SSLType IMAPS

IMAPStore main-remote
Account main

MaildirStore main-local
Path ~/mail/IRISA/
Inbox ~/mail/IRISA/INBOX
Flatten .

Channel main
Master :main-remote:
Slave :main-local:
Patterns INBOX Sent Drafts Trash Archives
Create Both
Sync Full
SyncState *
CopyArrivalDate yes
```

### Configure `msmtp`

Create a `~/.msmtprc` file:

```text
defaults
logfile  ~/.msmtp.log
port 587
tls on
tls_trust_file /etc/ssl/certs/ca-certificates.crt

account irisa
host smtp.inria.fr
from barais@irisa.fr
auth on
user barais
passwordeval gpg2 -q --for-your-eyes-only --no-tty -d ~/mail/zimbra.inria.fr.gpg

account default : irisa
```

### Configure `notmuch`

Run `notmuch setup` or edit `~/.notmuch-config`:

```ini
[database]
path=/home/barais/mail

[user]
name=Olivier Barais
primary_email=barais@irisa.fr
other_email=olivier.barais@gmail.com;

[new]
tags=unread;inbox;new;
ignore=.uidvalidity;.mbsyncstate;*.gpg

[search]
exclude_tags=deleted;spam;

[maildir]
synchronize_flags=true
```

### Configure `afew`

Follow the [afew official quickstart guide](https://afew.readthedocs.io/en/latest/quickstart.html).

**Crucial step:** Don't forget to create a `post-new` hook for notmuch to trigger `afew` automatically after syncing.

### 2. Verify your CLI Setup
Before launching the UI, ensure your backend tools are working properly:

*   **Sync your email:** `mbsync -Va`
*   **Query your index:** `notmuch search --limit=3 --format=json path:INBOX/**`
*   **Query an address:** `notmuch-addrlookup john`
*   **Send a test email:**
    ```bash
    printf "To: recipient@domain.com\nFrom: you@gmail.com\nSubject: Test\nHello." | msmtp recipient@domain.com
    ```


### 3. Automate Synchronization (Cron)

To fetch emails automatically without overlapping sync processes, use `cron` combined with `sem` (from GNU parallel).

Add this to your crontab (`crontab -e`):

```bash
# Sync every 2 minutes
*/2 * * * * sem --fg -j 1 --id mbsync 'mbsync -a' ; sem -j 1 --fg --id notmuch 'notmuch new --quiet'
```


## Install NotmuchTauri

Download the latest AppImage, deb or rpm from the **[Releases page](https://github.com/barais/notmuchtauri/releases)**. 

AppImages run directly on most Linux distributions without requiring installation. Just make it executable and run it:

```bash
chmod +x NotmuchTauri-x.x.x.AppImage
./NotmuchTauri-x.x.x.AppImage
```

---

### App Configuration (`config.json`)
Upon first launch (or in your configuration directory), configure the application using the following JSON structure:

```json
{
  "root_mail_dir": "/home/user/mail",
  "default_path": "/home/user/mail/IRISA/INBOX",
  "limit": 1000,
  "default_sent_folder": "Sent",
  "rmtmmail": "",
  "accounts": [
    {
      "id": "irisa",
      "label": "Work Account",
      "email": "user@irisa.fr",
      "sent_folder": "IRISA/Sent"
    },
    {
      "id": "personal",
      "label": "Personal Account",
      "email": "user@univ.fr"
    }
  ]
}
```
*Note: If you use [Remember The Milk](https://www.rememberthemilk.com/services/email/), put your specific RTM email address in the `rmtmmail` field. Otherwise, leave it blank.*



---

## 👨‍💻 Join the Development

This project is built with **Tauri**, **Vue 3** (using `<script setup>`), **TypeScript**, and **Tailwind CSS**.

### Recommended IDE Setup
*   [VS Code](https://code.visualstudio.com/)
*   [Vue - Official extension](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
*   [Tauri extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
*   [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Build from source
```bash
# Clone the repository
git clone https://github.com/barais/notmuchtauri.git
cd notmuchtauri

# Install frontend dependencies
npm install

# Run the app in development mode
npm run tauri dev

# Build the release executable / AppImage
npm run tauri build
```

## 🤝 Contribute
Contributions, issues, and feature requests are welcome! 
Feel free to check the [issues page](https://github.com/barais/notmuchtauri/issues) or fork the repository and submit a Pull Request.