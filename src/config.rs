#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateChunkKeyEncodingConfiguration {
    #[serde(alias = "format")]
    template: String,
    separator: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TemplateChunkKeyEncoding {
    template: String,
    pub(crate) interpolator: crate::Interpolator,
}

impl TemplateChunkKeyEncoding {
    pub fn try_new(
        template: impl Into<String>,
        separator: Option<impl Into<String>>,
    ) -> Result<Self, crate::errors::ParseError> {
        let template = template.into();
        let interpolator = crate::Interpolator::try_new(&template, separator.map(|s| s.into()))?;
        Ok(Self {
            template,
            interpolator,
        })
    }
}

impl TryFrom<TemplateChunkKeyEncodingConfiguration> for TemplateChunkKeyEncoding {
    type Error = crate::errors::ParseError;

    fn try_from(config: TemplateChunkKeyEncodingConfiguration) -> Result<Self, Self::Error> {
        Self::try_new(config.template, config.separator)
    }
}

impl From<TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: TemplateChunkKeyEncoding) -> Self {
        let separator = value
            .interpolator
            .has_catchall()
            .then(|| value.interpolator.sep.clone());
        Self {
            template: value.template,
            separator,
        }
    }
}

impl From<&TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: &TemplateChunkKeyEncoding) -> Self {
        let separator = value
            .interpolator
            .has_catchall()
            .then(|| value.interpolator.sep.clone());
        Self {
            template: value.template.clone(),

            separator,
        }
    }
}
