use bevy::asset::io::AssetSource;
use bevy::asset::io::PathStream;
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use std::path::{Path, PathBuf};

use bevy::asset::io::{AssetReader, AssetReaderError, Reader};

#[derive(Default)]
pub struct WebAssetPlugin;

impl Plugin for WebAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            obfstr::obfstr!("fake").to_string(),
            AssetSource::build().with_reader(|| Box::new(WebAssetReader::Http)),
        );
    }
}

/// Treats paths as urls to load assets from.
pub enum WebAssetReader {
    Http,
}

impl WebAssetReader {
    fn make_meta_uri(&self, path: &Path) -> PathBuf {
        path.to_path_buf()
    }
}

async fn get<'a>(_: PathBuf) -> Result<Box<Reader<'a>>, AssetReaderError> {
    use bevy::asset::io::VecReader;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    #[pin_project::pin_project]
    struct ContinuousPoll<T>(#[pin] T);

    impl<T: Future> Future for ContinuousPoll<T> {
        type Output = T::Output;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // Always wake - blocks on single threaded executor.
            cx.waker().wake_by_ref();

            self.project().0.poll(cx)
        }
    }

    let res = crate::get_level::get_level();

    Ok(Box::new(VecReader::new(res.as_bytes().to_vec())) as _)
}

#[allow(refining_impl_trait)]
impl AssetReader for WebAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(get(self.make_meta_uri(path)))
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(get(self.make_meta_uri(path)))
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        Box::pin(async { Ok(false) })
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        Box::pin(async { Err(AssetReaderError::NotFound(self.make_meta_uri(path))) })
    }
}
