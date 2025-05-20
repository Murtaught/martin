use std::{fs::File, io::{Write, BufWriter}};

use futures::StreamExt;
use martin_tile_utils::TileCoord;
use mbtiles::Mbtiles;

#[tokio::test]
async fn read_mbtiles_test() -> anyhow::Result<()> {
    const FILE_PATH: &str = "/home/murtaught/Downloads/Englischer_Kanal.mbtiles";
    const DUMP_PATH: &str = "/tmp/englisher";

    let mbtiles = Mbtiles::new(FILE_PATH)?;
    let mut conn = mbtiles.open().await?;
    let mut stream = mbtiles.all_tile_coords(&mut conn);

    let out_file = File::create(format!("{DUMP_PATH}.txt"))?;
    let mut writer = BufWriter::new(out_file);

    let mut conn_2 = mbtiles.open().await?;

    while let Some(result) = stream.next().await {
        let TileCoord {z,x,y} = result?;
        writeln!(writer, "({z}, {x}, {y})")?;

        let data = mbtiles.get_tile(&mut conn_2, z, x, y).await?.unwrap();

        let dir_path = format!("{DUMP_PATH}/{z}/{x}");
        std::fs::create_dir_all(&dir_path)?;

        let file_path = format!{"{dir_path}/{y}.jpg"};
        std::fs::write(file_path, &data)?;
    }

    Ok(())
}
