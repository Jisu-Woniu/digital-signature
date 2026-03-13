use std::{
    future::Future,
    io::{BufRead, BufReader, Read, Seek},
    path::Path,
};

use pgp::{
    composed::{Deserializable, SignedPublicKey, SignedSecretKey},
    errors::Error as PgpError,
    packet::{Packet, PacketParser, Signature},
};
use tokio::fs::File;

use crate::Result;

pub(crate) trait FromFile: Sized {
    fn try_from_reader(reader: impl BufRead + Read + Seek + Send + Unpin) -> Result<Self>;
    // fn try_from_async_reader(
    //     async_reader: impl AsyncRead + Send + Unpin + AsyncSeek,
    // ) -> Result<Self> {
    //     Self::try_from_reader(SyncIoBridge::new(async_reader))
    // }
    fn try_from_file(path: impl AsRef<Path> + Send) -> impl Future<Output = Result<Self>> {
        async move {
            let file = File::open(path).await?.into_std().await;
            Ok(Self::try_from_reader(BufReader::new(file))?)
        }
    }

    #[cfg(test)]
    fn try_from_armored_bytes(bytes: impl AsRef<[u8]> + Send + Unpin) -> Result<Self> {
        use std::io::Cursor;

        Self::try_from_reader(Cursor::new(bytes))
    }
}

macro_rules! impl_from_file {
    ($type:ty) => {
        impl FromFile for $type {
            fn try_from_reader(reader: impl BufRead + Read + Seek + Send + Unpin) -> Result<Self> {
                Ok(Self::from_armor_single(reader)?.0)
            }
        }
    };
}

impl_from_file!(SignedSecretKey);
impl_from_file!(SignedPublicKey);

impl FromFile for Signature {
    fn try_from_reader(reader: impl BufRead + Read + Send + Unpin + Seek) -> Result<Self> {
        let signature = PacketParser::new(reader)
            .find_map(|packet| {
                if let Ok(Packet::Signature(s)) = packet {
                    Some(s)
                } else {
                    None
                }
            })
            .ok_or(PgpError::NoMatchingPacket { backtrace: None })?;
        Ok(signature)
    }
}
