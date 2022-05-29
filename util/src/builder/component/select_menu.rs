//! Create a [`SelectMenu`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::{application::component::Component, channel::ReactionType, id::Id};
//! use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let component = Component::SelectMenu(
//!     SelectMenuBuilder::new("characters".to_string())
//!         .add_options(
//!             &mut vec![
//!                 SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
//!                     .default(true)
//!                     .emoji(ReactionType::Custom {
//!                         animated: false,
//!                         id: Id::new(754728776402993173),
//!                         name: Some("sparkle".to_string()),
//!                     })
//!                     .build(),
//!                 SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
//!                     .emoji(ReactionType::Custom {
//!                         animated: false,
//!                         id: Id::new(765306914153299978),
//!                         name: Some("rarsmile".to_string()),
//!                     })
//!                     .build(),
//!             ]
//!         ).validate()?.build()
//! );
//! # Ok(()) }
//! ```

use twilight_model::application::component::{select_menu::SelectMenuOption, SelectMenu};
use twilight_validate::component::{select_menu as validate_select_menu, ComponentValidationError};

/// Create a [`SelectMenu`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::{application::component::Component, channel::ReactionType, id::Id};
/// use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::SelectMenu(
///     SelectMenuBuilder::new("characters".to_string())
///         .add_options(
///             &mut vec![
///                 SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
///                     .default(true)
///                     .emoji(ReactionType::Custom {
///                         animated: false,
///                         id: Id::new(754728776402993173),
///                         name: Some("sparkle".to_string()),
///                     })
///                     .build(),
///                 SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
///                     .emoji(ReactionType::Custom {
///                         animated: false,
///                         id: Id::new(765306914153299978),
///                         name: Some("rarsmile".to_string()),
///                     })
///                     .build(),
///             ]
///         ).validate()?.build()
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    /// Create a new builder to construct a [`SelectMenu`].
    pub const fn new(custom_id: String) -> Self {
        Self(SelectMenu {
            custom_id,
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
        })
    }

    /// Consume the builder, returning a [`SelectMenu`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> SelectMenu {
        self.0
    }

    /// Ensure the select menu is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::select_menu`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        if let Err(source) = validate_select_menu(&self.0) {
            return Err(source);
        }

        Ok(self)
    }

    /// Set the minimum values for this select menu.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::SelectMenuBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .min_values(2)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values = Some(min_values);

        self
    }

    /// Set the maximum values for this select menu.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::SelectMenuBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .max_values(10)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[must_use = "builders have no effect if unused"]
    pub const fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values = Some(max_values);

        self
    }

    /// Set whether the select menu is enabled or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::SelectMenuBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .disable(true)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn disable(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    /// Set the placeholder for this select menu.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::SelectMenuBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .placeholder("this is a select menu".into())
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.0.placeholder = Some(placeholder);

        self
    }

    /// Add an option to this select menu.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .add_option(
    ///         SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
    ///             .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_option(mut self, option: SelectMenuOption) -> Self {
        self.0.options.push(option);

        self
    }

    /// Add multiple options to this select menu.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuBuilder::new("menu".into())
    ///     .add_options(
    ///         &mut vec![
    ///             SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
    ///                 .build(),
    ///             SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
    ///                 .build(),
    ///         ]
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_options(mut self, options: &mut Vec<SelectMenuOption>) -> Self {
        self.0.options.append(options);

        self
    }
}

impl TryFrom<SelectMenuBuilder> for SelectMenu {
    type Error = ComponentValidationError;

    /// Convert a select menu builder into a select menu, validating its contents.
    ///
    /// This is equivalent to calling [`SelectMenuBuilder::validate`], then
    /// [`SelectMenuBuilder::build`].
    fn try_from(builder: SelectMenuBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod test {
    use super::SelectMenuBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::application::component::SelectMenu;

    assert_impl_all!(SelectMenuBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(SelectMenu: TryFrom<SelectMenuBuilder>);

    #[test]
    fn test_select_menu_builder() {
        let select_menu = SelectMenuBuilder::new("a-menu".to_string()).build();

        let expected = SelectMenu {
            custom_id: "a-menu".to_string(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_select_menu_builder_disabled() {
        let select_menu = SelectMenuBuilder::new("a-menu".to_string())
            .disable(true)
            .build();

        let expected = SelectMenu {
            custom_id: "a-menu".to_string(),
            disabled: true,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_select_menu_builder_explicit_enabled() {
        let select_menu = SelectMenuBuilder::new("a-menu".to_string())
            .disable(false)
            .build();

        let expected = SelectMenu {
            custom_id: "a-menu".to_string(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_select_menu_builder_limited_values() {
        let select_menu = SelectMenuBuilder::new("a-menu".to_string())
            .max_values(10)
            .min_values(2)
            .build();

        let expected = SelectMenu {
            custom_id: "a-menu".to_string(),
            disabled: false,
            max_values: Some(10),
            min_values: Some(2),
            options: Vec::new(),
            placeholder: None,
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_select_menu_builder_placeholder() {
        let select_menu = SelectMenuBuilder::new("a-menu".to_string())
            .placeholder("I'm a placeholder".to_string())
            .build();

        let expected = SelectMenu {
            custom_id: "a-menu".to_string(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: Some("I'm a placeholder".to_string()),
        };

        assert_eq!(select_menu, expected);
    }
}
