use crate::error::Error;
use crate::state::AppState;
use gpui::*;
use std::path::Path;
use std::sync::Arc;
use util::ResultExt;

pub fn open<F>(state: Arc<AppState>, function: F, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let path = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    });

    cx.spawn(
        async move |_cx| match path.await.anyhow().and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path[0];
                *state.selected_file.lock().unwrap() = Some(path.to_path_buf());
                *state.is_selected.lock().unwrap() = Some(true);
                if let Err(err) = function(path, state) {
                    eprintln!("Open error: {:?}", err);
                }
            }
            Ok(None) => {}
            Err(_err) => {}
        },
    )
    .detach();
}

pub fn save<F>(state: Arc<AppState>, function: F, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let guard = state.directory.lock().unwrap();
    let directory = guard
        .as_ref()
        .map(|p| p.as_path())
        .unwrap_or(Path::new("."));
    let path = cx.prompt_for_new_path(directory, None);

    let state = state.clone();

    cx.spawn(
        async move |_cx| match path.await.anyhow().and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path;
                if let Err(err) = function(path, state) {
                    eprintln!("Open error: {:?}", err);
                }
            }
            Ok(None) => {}
            Err(_err) => {}
        },
    )
    .detach();
}
