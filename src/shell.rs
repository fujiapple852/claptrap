#[derive(clap::ValueEnum, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Friendly Interactive `SHell` (fish)
    Fish,
    /// `PowerShell`
    #[allow(clippy::enum_variant_names)]
    PowerShell,
    /// Z `SHell` (zsh)
    Zsh,
}
