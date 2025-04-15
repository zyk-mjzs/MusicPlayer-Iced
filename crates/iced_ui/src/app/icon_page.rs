use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length};
use iced_font_awesome::{
    FontAwesomeBrands, FontAwesomeRegular, FontAwesomeSolid, brands, regular,
    solid,
};

use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

use super::index::Message;

pub fn icon_page() -> Element<'static, Message> {
    let solid_icons = FontAwesomeSolid::iter()
        .map(|icon| {
            container(
                column![
                    solid(icon).size(24),
                    // text(format!("{:?}", icon)).size(12)
                ]
                .spacing(5),
            )
            .width(100)
            .height(100)
        })
        .collect::<Vec<_>>();

    let regular_icons = FontAwesomeRegular::iter()
        .map(|icon| {
            container(
                column![
                    regular(icon).size(24),
                    // text(format!("{:?}", icon)).size(12)
                ]
                .spacing(5),
            )
            .width(100)
            .height(100)
        })
        .collect::<Vec<_>>();

    let brands_icons = FontAwesomeBrands::iter()
        .map(|icon| {
            container(
                column![
                    brands(icon).size(24),
                    // text(format!("{:?}", icon)).size(12)
                ]
                .spacing(5),
            )
            .width(100)
            .height(100)
        })
        .collect::<Vec<_>>();

    column![
        text("Solid Icons").size(24),
        scrollable(
            row(solid_icons
                .into_iter()
                .map(Element::from)
                .collect::<Vec<_>>())
            .spacing(10)
            .width(Length::Fill)
            .wrap(),
        ),
        text("Regular Icons").size(24),
        scrollable(
            row(regular_icons
                .into_iter()
                .map(Element::from)
                .collect::<Vec<_>>())
            .spacing(10)
            .width(Length::Fill)
            .wrap()
        ),
        text("Brands Icons").size(24),
        scrollable(
            row(brands_icons
                .into_iter()
                .map(Element::from)
                .collect::<Vec<_>>())
            .spacing(10)
            .width(Length::Fill)
            .wrap()
        )
    ]
    .padding(20)
    .spacing(20)
    .width(Length::Fill)
    .into()
}
