mod notion; // Do I need to make this pub if its the crate *root*?

/*
 * Entry point to the Notion cli.
 */
fn main() {
    
    /* Program flow and TODO */
    // Take readme.md path as program input
    // Check if path actually contains a markdown file 
    // If not, exit the program and inform the user 
    // Try to open the file with read-only access 
    // If failed, exit the program and inform the user 
    // Connect to Notion client
    // If failed, exit the program and inform the user
    // Get the top level workspaces and ask the user which one.
    // Get the pages and ask the user which one.
    // Check the page submittable sections and ask the user which one (append to table or new page only for now)
    // Ask the user where to which workspace, page or table to submit to.
    // Convert the file into Notion page format 
    // Wrap the page formt with necessary metadata 
    // Send to specified Notion page via API.

}
