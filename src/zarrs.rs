use zarrs_chunk_key_encoding::{ChunkKeyEncoding, ChunkKeyEncodingPlugin, ChunkKeyEncodingTraits};
use zarrs_metadata::v3::MetadataV3;
use zarrs_metadata::{Configuration, ConfigurationSerialize};
use zarrs_plugin::{PluginConfigurationInvalidError, PluginCreateError};
use zarrs_storage::StoreKey;

use crate::errors::ParseError;

zarrs_plugin::impl_extension_aliases!(TemplateChunkKeyEncoding, v3: "template", ["zarrs:template"]);

// Register the chunk key encoding.
inventory::submit! {
    ChunkKeyEncodingPlugin::new::<TemplateChunkKeyEncoding>()
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateChunkKeyEncodingConfiguration {
    format: String,
    separator: Option<String>,
}

impl ConfigurationSerialize for TemplateChunkKeyEncodingConfiguration {}

#[derive(Debug, Clone)]
pub struct TemplateChunkKeyEncoding {
    format: String,
    interpolator: crate::Interpolator,
}

impl TemplateChunkKeyEncoding {
    pub fn try_new(
        format: impl Into<String>,
        separator: Option<impl Into<String>>,
    ) -> Result<Self, crate::errors::ParseError> {
        let format = format.into();
        let interpolator = crate::Interpolator::try_new(&format, separator.map(|s| s.into()))?;
        Ok(Self {
            format,
            interpolator,
        })
    }
}

impl TryFrom<TemplateChunkKeyEncodingConfiguration> for TemplateChunkKeyEncoding {
    type Error = crate::errors::ParseError;

    fn try_from(config: TemplateChunkKeyEncodingConfiguration) -> Result<Self, Self::Error> {
        Self::try_new(config.format, config.separator)
    }
}

impl From<TemplateChunkKeyEncoding> for TemplateChunkKeyEncodingConfiguration {
    fn from(value: TemplateChunkKeyEncoding) -> Self {
        let separator = value
            .interpolator
            .has_catchall()
            .then(|| value.interpolator.sep.clone());
        Self {
            format: value.format,
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
            format: value.format.clone(),

            separator,
        }
    }
}

impl ChunkKeyEncodingTraits for TemplateChunkKeyEncoding {
    fn create(metadata: &MetadataV3) -> Result<ChunkKeyEncoding, PluginCreateError>
    where
        Self: Sized,
    {
        let configuration: TemplateChunkKeyEncodingConfiguration =
            metadata.to_typed_configuration()?;
        let template = TemplateChunkKeyEncoding::try_from(configuration)?;
        Ok(template.into())
    }

    fn configuration(&self) -> Configuration {
        TemplateChunkKeyEncodingConfiguration::from(self).into()
    }

    fn encode(&self, chunk_grid_indices: &[u64]) -> StoreKey {
        let key = self
            .interpolator
            .interpolate(chunk_grid_indices)
            .expect("Failed to interpolate chunk key");
        StoreKey::new(key).expect("Interpolated string is not a valid chunk key")
    }
}

impl From<ParseError> for PluginCreateError {
    fn from(err: ParseError) -> Self {
        PluginCreateError::ConfigurationInvalid(PluginConfigurationInvalidError::new(
            err.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use zarrs_plugin::ExtensionName;

    use super::*;
    use crate::tests::FORMAT;

    #[test]
    fn can_deser() {
        let json = serde_json::json!({
            "name": "template",
            "configuration": {
                "format": FORMAT,
                "separator": ":"
            }
        });
        let meta = MetadataV3::deserialize(json).expect("failed to deserialize metadata");
        let config: TemplateChunkKeyEncodingConfiguration = meta
            .to_typed_configuration()
            .expect("failed to deserialize typed configuration");
        let _encoding =
            TemplateChunkKeyEncoding::try_from(config).expect("failed to create encoder");
    }

    #[test]
    fn can_ser() {
        let encoding =
            TemplateChunkKeyEncoding::try_new(FORMAT, Some(":")).expect("failed to create encoder");
        let config: TemplateChunkKeyEncodingConfiguration = (&encoding).into();
        let meta = MetadataV3::new_with_serializable_configuration(
            encoding
                .name(zarrs_plugin::ZarrVersion::V3)
                .unwrap()
                .to_string(),
            &config,
        )
        .expect("failed to serialize typed configuration");
        let json = serde_json::to_value(&meta).expect("failed to serialize metadata");
        let expected_json = serde_json::json!(
            {
                "name": "template",
                "configuration": {
                    "format": FORMAT,
                    "separator": ":"
                }
            }
        );
        assert_eq!(json, expected_json);
    }
}
