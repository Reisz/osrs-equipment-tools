//! Displays buttons to allow switching region filters.

use enum_iterator::IntoEnumIterator;
use regions::vars::Region;
use seed::prelude::*;
use seed::*;

use crate::model::{
    region_filter::{RegionFilter, TrailblazerMsg},
    Msg,
};

/// Create the DOM according to the [`Model`].
pub fn view(filter: &RegionFilter) -> Node<Msg> {
    div![
        span![
            C!["button"],
            IF!(filter.enabled() => C!["checked"]),
            ev(Ev::Click, move |_| Msg::Trailblazer(
                TrailblazerMsg::ToggleEnabled
            )),
            "Enable"
        ],
        Region::into_enum_iter().map(|r| view_button(filter, r))
    ]
}

fn view_button(filter: &RegionFilter, region: Region) -> Node<Msg> {
    span![
        C!["button"],
        IF!(filter.filter()[region] => C!["checked"]),
        ev(Ev::Click, move |_| Msg::Trailblazer(
            TrailblazerMsg::ToggleRegion(region)
        )),
        format!("{:?}", region)
    ]
}
