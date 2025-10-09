use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

pub async fn transfer_all(source_account: &str, target_account: &str) -> Result<()> {
    let source_client = crate::spotify::client::SpotifyClient::new(source_account)?;
    let target_client = crate::spotify::client::SpotifyClient::new(target_account)?;

    let multi = MultiProgress::new();
    
    // Transfer liked songs
    println!("\n🎵 Transferring Liked Songs...");
    transfer_liked_songs(&source_client, &target_client, &multi).await?;
    
    // Transfer playlists
    println!("\n� Transferring Playlists...");
    transfer_playlists(&source_client, &target_client, &multi).await?;

    println!("\n✅ Transfer complete!");
    
    Ok(())
}

async fn transfer_liked_songs(
    source: &crate::spotify::client::SpotifyClient,
    target: &crate::spotify::client::SpotifyClient,
    multi: &MultiProgress,
) -> Result<()> {
    let pb = multi.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Fetching liked songs...");

    let liked_songs = source.get_liked_songs().await?;
    let total = liked_songs.len();
    
    pb.finish_with_message(format!("Found {} liked songs", total));
    
    if total == 0 {
        return Ok(());
    }

    let pb = multi.add(ProgressBar::new(total as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message("Adding liked songs");

    let track_ids: Vec<String> = liked_songs.iter().map(|t| t.id.clone()).collect();
    
    // Add in chunks and update progress
    for chunk in track_ids.chunks(50) {
        target.add_liked_songs(chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message(format!("✓ Added {} liked songs", total));
    
    Ok(())
}

async fn transfer_playlists(
    source: &crate::spotify::client::SpotifyClient,
    target: &crate::spotify::client::SpotifyClient,
    multi: &MultiProgress,
) -> Result<()> {
    let pb = multi.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Fetching playlists...");

    let playlists = source.get_playlists().await?;
    let total = playlists.len();
    
    pb.finish_with_message(format!("Found {} playlists", total));
    
    if total == 0 {
        return Ok(());
    }

    let pb = multi.add(ProgressBar::new(total as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message("Creating playlists");

    for playlist in playlists {
        pb.set_message(format!("Processing: {}", playlist.name));
        
        // Get tracks from source playlist
        let tracks = source.get_playlist_tracks(&playlist.id).await?;
        
        if tracks.is_empty() {
            pb.inc(1);
            continue;
        }

        // Create playlist in target
        let new_playlist_id = target
            .create_playlist(
                &playlist.name,
                playlist.description.as_deref(),
                playlist.public,
            )
            .await?;

        // Add tracks to new playlist
        let track_uris: Vec<String> = tracks.iter().map(|t| t.uri.clone()).collect();
        target.add_tracks_to_playlist(&new_playlist_id, &track_uris).await?;

        pb.inc(1);
    }

    pb.finish_with_message(format!("✓ Created {} playlists", total));
    
    Ok(())
}