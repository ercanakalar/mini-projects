#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
fn token_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("road_planner")
        .join("access_token")
}

pub async fn save_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = LocalStorage::set("access_token", token);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = token_path();

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let _ = std::fs::write(path, token);
    }
}

pub async fn load_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        LocalStorage::get("access_token").ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string(token_path()).ok()
    }
}

pub async fn clear_token() {
    #[cfg(target_arch = "wasm32")]
    {
        LocalStorage::delete("access_token");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = std::fs::remove_file(token_path());
    }
}

pub async fn save_refresh_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = LocalStorage::set("refresh_token", token);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = refresh_token_path();

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let _ = std::fs::write(path, token);
    }
}

pub async fn load_refresh_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        LocalStorage::get("refresh_token").ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string(refresh_token_path()).ok()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn refresh_token_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("road_planner")
        .join("refresh_token")
}
