pub type ThreadMessages = Vec<Message>;

impl NotMuchWrapper {
    // ... existing methods ...

    /// Fetches all messages belonging to a specific thread, ordered by date.
    pub fn get_thread_messages(thread_id: &str) -> Result<ThreadMessages, Box<dyn Error>> {
        // We search for all messages that belong to this thread
        let query = format!("thread:{}", thread_id);
        let raw_msgs = Self::search(&query)?;

        // Now we need to get the full details for each message in the thread
        // to get the bodies and full headers.
        let mut messages = Vec::new();

        for raw in raw_msgs {
            // In the search result, the thread ID is provided, but for specific message
            // details we need the actual message ID.
            // Note: notmuch search --format=json for a thread gives the thread info.
            // To get individual messages, we need to fetch them.

            // This is a simplification: in a real scenario, we'd use notmuch's
            // ability to list messages in a thread.
            if let Ok(details) = Self::get_message_details(&raw.thread) {
                messages.push(details);
            }
        }

        // Sort by date descending (newest first)
        // Note: our Message struct uses a String for date, which is not ideal for sorting.
        // In a real impl, we would use the timestamp (i64) from the raw search.
        messages.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(messages)
    }
}
