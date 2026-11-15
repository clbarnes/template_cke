use monostate::MustBe;

use crate::TemplateChunkKeyEncoding;

/// `{"name": "template", "configuration": { ... }}` data structure.
///
/// Should be created from a valid [TemplateChunkKeyEncoding].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateChunkKeyEncodingMetadata {
    name: MustBe!("template"),
    configuration: TemplateChunkKeyEncodingConfiguration,
}

impl TemplateChunkKeyEncodingMetadata {
    /// Create metadata without validating the template.
    pub fn new_unchecked(
        template: impl Into<String>,
        separator: Option<impl Into<String>>,
    ) -> Self {
        TemplateChunkKeyEncodingConfiguration::new_unchecked(template, separator).into()
    }
}

impl From<TemplateChunkKeyEncodingMetadata> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: TemplateChunkKeyEncodingMetadata) -> Self {
        value.configuration
    }
}

impl From<TemplateChunkKeyEncodingConfiguration> for TemplateChunkKeyEncodingMetadata {
    fn from(value: TemplateChunkKeyEncodingConfiguration) -> Self {
        Self {
            name: MustBe!("template"),
            configuration: value,
        }
    }
}

impl TryFrom<TemplateChunkKeyEncodingMetadata> for TemplateChunkKeyEncoding {
    type Error = crate::errors::ParseError;

    fn try_from(metadata: TemplateChunkKeyEncodingMetadata) -> Result<Self, Self::Error> {
        Self::try_from(metadata.configuration)
    }
}

impl From<TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingMetadata {
    fn from(value: TemplateChunkKeyEncoding) -> Self {
        Self::from(&value)
    }
}

impl From<&TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingMetadata {
    fn from(value: &TemplateChunkKeyEncoding) -> Self {
        TemplateChunkKeyEncodingConfiguration::from(value).into()
    }
}

/// Configuration object data structure.
///
/// Should be created from a valid [TemplateChunkKeyEncoding].
///
///
/// ```jsonc
/// {"name": "template", "configuration": { ... }}
/// //                                    ^^^^^^^
/// //                                  this object
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateChunkKeyEncodingConfiguration {
    #[cfg_attr(feature = "serde", serde(alias = "format"))]
    template: String,
    separator: Option<String>,
}

impl TemplateChunkKeyEncodingConfiguration {
    /// Create a configuration without validating the template.
    pub fn new_unchecked(
        template: impl Into<String>,
        separator: Option<impl Into<String>>,
    ) -> Self {
        Self {
            template: template.into(),
            separator: separator.map(Into::into),
        }
    }
}

impl TryFrom<TemplateChunkKeyEncodingConfiguration> for TemplateChunkKeyEncoding {
    type Error = crate::errors::ParseError;

    fn try_from(config: TemplateChunkKeyEncodingConfiguration) -> Result<Self, Self::Error> {
        Self::try_new(&config.template, config.separator)
    }
}

impl From<TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: TemplateChunkKeyEncoding) -> Self {
        Self::from(&value)
    }
}

impl From<&TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: &TemplateChunkKeyEncoding) -> Self {
        let separator = value.has_catchall().then(|| value.sep.clone());
        Self {
            template: value.display().to_string(),
            separator,
        }
    }
}
