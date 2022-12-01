/// Rust bindings for devices that support the Awair Local API.
///
/// The Awair Local API is documented here:
/// <https://support.getawair.com/hc/en-us/articles/360049221014-Awair-Element-Local-API-Feature>
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents the errors that can occur when retrieving search results.
#[derive(Debug, Error)]
pub enum Error {
    /// The URL requested by the user is valid, but unusable.
    #[error("invalid API URL: cannot be a valid base")]
    InvalidBase(String),
    /// An API URL is invalid.
    #[error("invalid API URL")]
    InvalidUrl(#[from] url::ParseError),
    /// An request error occurred.
    #[error("request error")]
    Request(#[from] reqwest::Error),
}

/// Represents a sample of air quality data taken from an Awair
/// device's Local API.
#[derive(Debug, Serialize, Deserialize)]
pub struct AirData {
    /// The time reported by the device's internal clock.
    pub timestamp: DateTime<Utc>,
    /// The Awair Score, from 0-100.
    pub score: u8,
    /// The dew point, in degrees Celsius.
    pub dew_point: f32,
    /// The dry bulb temperature, in degrees Celsius.
    #[serde(rename = "temp")]
    pub temperature: f32,
    /// The relative humidity, as a percent.
    #[serde(rename = "humid")]
    pub humidity: f32,
    /// The absolute humidity, as a percent.
    #[serde(rename = "abs_humid")]
    pub absolute_humidity: f32,
    /// The CO2 reading, in parts per million.
    pub co2: u32,
    #[serde(rename = "co2_est")]
    /// The VOC sensor's estimated CO2 reading, in parts per million.
    pub estimated_co2: u32,
    /// The VOC sensor's CO2 baseline (unitless).
    #[serde(rename = "co2_est_baseline")]
    pub estimated_co2_baseline: u32,
    /// The TVOC reading, in parts per billion.
    pub voc: u32,
    /// The TVOC sensor's VOC baseline (unitless).
    pub voc_baseline: u32,
    /// The TVOC sensor's H2 (hydrogen gas) reading (unitless).
    pub voc_h2_raw: u32,
    /// The TVOC sensor's ethanol gas reading (unitless).
    pub voc_ethanol_raw: u32,
    /// The PM2.5 reading (in microns per cubic meter)
    pub pm25: u32,
    /// The PM10 reading (in microns per cubic meter)
    #[serde(rename = "pm10_est")]
    pub estimated_pm10: u32,
}

/// The Awair device's LED configuration state, as returned from
/// the Local API.
#[derive(Debug, Serialize, Deserialize)]
pub struct LedConfig {
    /// The LED's operating mode.
    pub mode: String,
    /// The LED's brightness (unknown units).
    pub brightness: u32,
}

/// Represents a Awair device's active configuration, as
/// returned from the Local API.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// The Awair device's ID.
    ///
    /// NOTE: The Local API refers to this as `device_uuid`, but it isn't
    /// formatted as a normal UUID and there's no indication that it's
    /// intended to be universally unique.
    #[serde(rename = "device_uuid")]
    pub device_id: String,
    /// The MAC address of the WiFi network that the Awair is connected to.
    pub wifi_mac: String,
    /// The SSID of the WiFi network.
    pub ssid: String,
    /// The Awair's IP address on the network.
    pub ip: String,
    /// The network's mask, in dotted quad format.
    pub netmask: String,
    /// The network's gateway IP address.
    pub gateway: String,
    /// The Awair's active firmware version.
    #[serde(rename = "fw_version")]
    pub firmware_version: String,
    /// The Awair's configured timezone, as a TZ database name.
    pub timezone: String,
    /// The Awair's current display mode.
    pub display: String,
    /// The Awair's current LED configuration.
    pub led: LedConfig,
    /// (Presumably) the TVOC sensor's feature set (unknown format).
    pub voc_feature_set: u32,
}

/// Represents a connection to an Awair device.
#[derive(Debug)]
pub struct Awair {
    api_base: url::Url,
    http: reqwest::blocking::Client,
}

impl Awair {
    /// Create a new client capable of talking to an Awair's Local API.
    pub fn new(api_base: &str) -> Result<Self, Error> {
        let api_base = url::Url::parse(api_base)?;
        if api_base.cannot_be_a_base() {
            return Err(Error::InvalidBase(api_base.into()));
        }

        Ok(Self {
            api_base,
            http: reqwest::blocking::Client::new(),
        })
    }

    /// Poll the Awair for its latest air quality data.
    pub fn poll(&self) -> Result<AirData, Error> {
        let latest = self.api_base.join("/air-data/latest")?;

        Ok(self
            .http
            .get(latest)
            .send()?
            .error_for_status()?
            .json::<AirData>()?)
    }

    pub fn config(&self) -> Result<DeviceConfig, Error> {
        let config = self.api_base.join("/settings/config/data")?;

        Ok(self
            .http
            .get(config)
            .send()?
            .error_for_status()?
            .json::<DeviceConfig>()?)
    }
}
