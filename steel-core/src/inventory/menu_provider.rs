//! Menu instance trait for dynamic menu dispatch.

use steel_registry::menu_type::MenuTypeRef;
use steel_utils::text::TextComponent;

use crate::inventory::menu::Menu;

/// Trait for menu instances that can be opened by players.
///
/// This extends `Menu` with the additional information needed to send
/// the open screen packet: menu type and display title.
pub trait MenuInstance: Menu + Send + Sync {
    /// Returns the menu type for the open screen packet.
    fn menu_type(&self) -> MenuTypeRef;

    /// Returns the container ID for this menu.
    fn container_id(&self) -> u8;

    /// Returns the display title for this menu.
    fn title(&self) -> TextComponent;
}
