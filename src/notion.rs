// TODO: This should be a library crate! My binary crate will be a simple executable that runs
// this.

/*
 * Notion Client to interface with the provided API.
 */
pub struct NotionClient {
    // Data for the client itself
    pub is_connected: bool,

    // User notion data
    pub username: String,
    pub email: String,
    pub http_client: HTTPClient,
}

impl NotionClient {
    /*
     * Connects to your Notion account if not already connected.
     * Maintains a persistent HTTP connection by default.
     * Note that connection properties cannot be modified afterwards,
     * and must be disconnected then reconnected.
     */
    // TODO: Any "success or fail" type? What to return here? Perhaps a recoverable error
    // Result<<>, E>
    pub fn connect(&mut self) -> Result<(), bool> {
        if self.is_connected {
            return Err(false);
        }

        todo!();

        self.is_connected = true;
        return Ok(());
    }

    /*
     * Disconnects the HTTP connection if not already disconnected.
     */
    pub fn disconnect(&mut self) -> Result<(), bool> {
        if !self.is_connected {
            return Err(false);
        }

        todo!();

        self.is_connected = false;
        return Ok(());
    }

    /*
     *
     */
    pub fn get_workspaces() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn connects_to_notion_integration() {
        todo!();
    }
    
    #[test]
    fn disconnects_from_notion_integration() {
        todo!();
    }

    #[test]
    fn no_double_connect() {
        todo!();
    }

    #[test]
    fn no_double_disconnect() {
        todo!();
    }
}
