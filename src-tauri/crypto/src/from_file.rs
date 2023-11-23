use std::{io::Read, path::Path};

use async_trait::async_trait;
use pgp::{
    errors::Error as PgpError,
    packet::{Packet, PacketParser},
    Deserializable, Signature, SignedPublicKey, SignedSecretKey,
};
use tokio::{
    fs::File,
    io::{AsyncRead, BufReader},
};
use tokio_util::io::SyncIoBridge;

use crate::Result;

#[async_trait]
pub(crate) trait FromFile: Sized {
    fn try_from_reader(reader: impl Read + Send + Unpin) -> Result<Self>;
    fn try_from_async_reader(async_reader: impl AsyncRead + Send + Unpin) -> Result<Self> {
        Self::try_from_reader(SyncIoBridge::new(async_reader))
    }
    async fn try_from_file(path: impl AsRef<Path> + Send) -> Result<Self> {
        let file = File::open(path).await?;
        Ok(Self::try_from_async_reader(BufReader::new(file))?)
    }
}

macro_rules! impl_from_file {
    ($type:ty) => {
        impl FromFile for $type {
            fn try_from_reader(reader: impl Read + Send + Unpin) -> Result<Self> {
                Ok(Self::from_bytes(reader)?)
            }
        }
    };
}

impl_from_file!(SignedSecretKey);
impl_from_file!(SignedPublicKey);

impl FromFile for Signature {
    fn try_from_reader(reader: impl Read + Send + Unpin) -> Result<Self> {
        let signature = PacketParser::new(reader)
            .find_map(|packet| {
                if let Ok(Packet::Signature(s)) = packet {
                    Some(s)
                } else {
                    None
                }
            })
            .ok_or(PgpError::MissingPackets)?;
        Ok(signature)
    }
}
