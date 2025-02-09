use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone)]
pub enum LoaderType {
    Bukkit,
    Bungeecord,
    Canvas,
    Datapack,
    Fabric,
    Folia,
    Forge,
    Iris,
    Liteloader,
    Minecraft,
    Modloader,
    Neoforge,
    Optifine,
    Paper,
    Purpur,
    Quilt,
    Rift,
    Spigot,
    Sponge,
    Vanilla,
    Velocity,
    Waterfall,
}

/// Implement conversion from `LoaderType` to `&str`
impl From<LoaderType> for &'static str {
    fn from(loader: LoaderType) -> Self {
        match loader {
            LoaderType::Bukkit => "bukkit",
            LoaderType::Bungeecord => "bungeecord",
            LoaderType::Canvas => "canvas",
            LoaderType::Datapack => "datapack",
            LoaderType::Fabric => "fabric",
            LoaderType::Folia => "folia",
            LoaderType::Forge => "forge",
            LoaderType::Iris => "iris",
            LoaderType::Liteloader => "liteloader",
            LoaderType::Minecraft => "minecraft",
            LoaderType::Modloader => "modloader",
            LoaderType::Neoforge => "neoforge",
            LoaderType::Optifine => "optifine",
            LoaderType::Paper => "paper",
            LoaderType::Purpur => "purpur",
            LoaderType::Quilt => "quilt",
            LoaderType::Rift => "rift",
            LoaderType::Spigot => "spigot",
            LoaderType::Sponge => "sponge",
            LoaderType::Vanilla => "vanilla",
            LoaderType::Velocity => "velocity",
            LoaderType::Waterfall => "waterfall",
        }
    }
}

impl LoaderType {
    pub fn as_str(&self) -> &str {
        match self {
            LoaderType::Bukkit => "bukkit",
            LoaderType::Bungeecord => "bungeecord",
            LoaderType::Canvas => "canvas",
            LoaderType::Datapack => "datapack",
            LoaderType::Fabric => "fabric",
            LoaderType::Folia => "folia",
            LoaderType::Forge => "forge",
            LoaderType::Iris => "iris",
            LoaderType::Liteloader => "liteloader",
            LoaderType::Minecraft => "minecraft",
            LoaderType::Modloader => "modloader",
            LoaderType::Neoforge => "neoforge",
            LoaderType::Optifine => "optifine",
            LoaderType::Paper => "paper",
            LoaderType::Purpur => "purpur",
            LoaderType::Quilt => "quilt",
            LoaderType::Rift => "rift",
            LoaderType::Spigot => "spigot",
            LoaderType::Sponge => "sponge",
            LoaderType::Vanilla => "vanilla",
            LoaderType::Velocity => "velocity",
            LoaderType::Waterfall => "waterfall",
        }
    }
}
