use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;

use crate::context::AppContext;
use crate::util::request::send_client_request;

use super::change_directory;

pub fn open(context: &mut AppContext) -> DiziResult<()> {
    if let Some(entry) = context.curr_list_ref().and_then(|s| s.curr_entry_ref()) {
        if entry.file_path().is_dir() {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
        } else {
            let request = ClientRequest::PlayerFilePlay {
                path: entry.file_path().to_path_buf(),
            };
            send_client_request(context, &request)?;
        }
    }
    Ok(())
}
