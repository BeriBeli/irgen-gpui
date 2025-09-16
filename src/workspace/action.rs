use crate::error::Error;
use crate::state::AppState;
use gpui::*;
use std::path::Path;
use std::sync::Arc;
use util::ResultExt;

pub fn open<F>(state: Arc<AppState>, function: F, cx: &mut App) -> anyhow::Result<(), Error>
where
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let path = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    });

    cx.spawn(async move |_cx| -> anyhow::Result<(), Error> {
        match path.await.anyhow().and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path[0];
                *state.selected_file.lock().unwrap() = Some(path.to_path_buf());
                *state.is_selected.lock().unwrap() = Some(true);
                function(path, state)?;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err.into()),
        }
    })
    .detach();
    Ok(())
}

pub fn save<F>(state: Arc<AppState>, function: F, cx: &mut App) -> anyhow::Result<(), Error>
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

    cx.spawn(async move |_cx| -> anyhow::Result<(), Error> {
        match path.await.anyhow().and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path;
                function(path, state)?;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err.into()),
        }
    })
    .detach();
    Ok(())
}
