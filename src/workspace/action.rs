use crate::error::Error;
use crate::services::AppState;
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
    let path = cx.prompt_for_new_path(Path::new("./"), None);

    cx.spawn(async move |_cx| match path.await {
        Ok(path) => {
            let path = &path.ok().unwrap().unwrap();
            if let Err(err) = function(path, state) {
                eprintln!("Open error: {:?}", err);
            }
        }
        Err(_err) => {}
    })
    .detach();
}
