use godot::prelude::*;

mod player;

struct GarlicFarmExtension;

#[gdextension]
unsafe impl ExtensionLibrary 
for GarlicFarmExtension {}