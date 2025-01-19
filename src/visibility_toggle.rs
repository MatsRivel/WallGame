use super::*;
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[require(WireFrame)]
pub enum GizmoOutlineToggle {
    Visible,
    Invisible,
}
impl Default for GizmoOutlineToggle{
    fn default() -> Self {
        Self::Invisible
    }
}
impl GizmoOutlineToggle {
    pub fn is_visible(&self) -> bool {
        match self {
            GizmoOutlineToggle::Visible => true,
            GizmoOutlineToggle::Invisible => false,
        }
    }
    pub fn is_invisible(&self) -> bool {
        match self {
            GizmoOutlineToggle::Visible => false,
            GizmoOutlineToggle::Invisible => true,
        }
    }
}
pub fn tag_visible_on_hover(
    hit: Trigger<Pointer<Over>>,
    mut frame_query: Query<(Entity, &mut GizmoOutlineToggle), With<IsHoverable>>,
) {
    for (_, mut visibility) in frame_query.iter_mut().filter(|(e, _)| *e == hit.target) {
        *visibility = GizmoOutlineToggle::Visible;
    }
}

pub fn tag_invisible_on_hover_end(
    hit: Trigger<Pointer<Out>>,
    mut frame_query: Query<(Entity, &mut GizmoOutlineToggle), With<IsHoverable>>,
) {
    for (_, mut visibility) in frame_query.iter_mut().filter(|(e, _)| *e == hit.target) {
        *visibility = GizmoOutlineToggle::Invisible;
    }
}
