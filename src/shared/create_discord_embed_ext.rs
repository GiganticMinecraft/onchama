use chrono::Utc;
use poise::serenity_prelude::{Color, CreateEmbed};

pub trait CreateDiscordEmbedExt {
    fn current_timestamp(&mut self) -> &mut Self;
    fn simple_color(&mut self) -> &mut Self;
    fn success_color(&mut self) -> &mut Self;
    fn failure_color(&mut self) -> &mut Self;
    fn custom_field<T, U>(&mut self, name: T, value: U, inline: bool) -> &mut Self
    where
        T: ToString,
        U: ToString;
    fn custom_fields<T, U, It>(&mut self, fields: It) -> &mut Self
    where
        T: ToString,
        U: ToString,
        It: IntoIterator<Item = (T, U, bool)>,
    {
        fields.into_iter().for_each(|(name, value, inline)| {
            self.custom_field(name, value, inline);
        });

        self
    }
}

impl CreateDiscordEmbedExt for CreateEmbed {
    fn current_timestamp(&mut self) -> &mut Self {
        self.timestamp(Utc::now().to_rfc3339())
    }

    fn simple_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(255, 255, 255))
    }

    fn success_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(50, 173, 240))
    }

    fn failure_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(245, 93, 93))
    }

    fn custom_field<T, U>(&mut self, name: T, value: U, inline: bool) -> &mut Self
    where
        T: ToString,
        U: ToString,
    {
        let name = name.to_string();
        let name = if name.is_empty() {
            "-".to_string()
        } else {
            name
        };

        let value = value.to_string();
        let value = if value.is_empty() {
            "-".to_string()
        } else {
            value
        };

        self.field(name, value, inline)
    }
}
