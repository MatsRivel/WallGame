use super::*;
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IsVisible {
    Visible,
    Invisible,
}
impl IsVisible{
    pub fn is_visible(&self)->bool{
        match self{
            IsVisible::Visible => true,
            IsVisible::Invisible => false,
        }
    }
    pub fn is_invisible(&self)->bool{
        match self{
            IsVisible::Visible => false,
            IsVisible::Invisible => true,
        }
    }
}
pub fn tag_visible_on_hover(
    hit: Trigger<Pointer<Over>>,
    mut frame_query: Query<(Entity, &mut IsVisible), With<GridType>>,
) {
    for (_, mut visibility) in frame_query.iter_mut().filter(|(e, _)| *e == hit.target) {
        *visibility = IsVisible::Visible;
    }
}

pub fn tag_invisible_on_hover_end(
    hit: Trigger<Pointer<Out>>,
    mut frame_query: Query<(Entity, &mut IsVisible), With<GridType>>,
) {
    for (_, mut visibility) in frame_query.iter_mut().filter(|(e, _)| *e == hit.target) {
        *visibility = IsVisible::Invisible;
    }
}
