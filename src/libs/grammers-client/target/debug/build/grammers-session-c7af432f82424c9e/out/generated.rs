
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The schema layer from which the definitions were generated.
pub const LAYER: i32 = 2;

/// This module contains all of the bare types, each
/// represented by a `struct`. All of them implement
/// [`Identifiable`], [`Serializable`] and [`Deserializable`].
///
/// [`Identifiable`]: ../trait.Identifiable.html
/// [`Serializable`]: ../trait.Serializable.html
/// [`Deserializable`]: ../trait.Deserializable.html
#[allow(clippy::cognitive_complexity, clippy::identity_op, clippy::unreadable_literal)]
pub mod types {
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct ChannelState {
        pub channel_id: i64,
        pub pts: i32,
    }
    impl crate::Identifiable for ChannelState {
        const CONSTRUCTOR_ID: u32 = 3266377933;
    }
    impl crate::Serializable for ChannelState {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            self.channel_id.serialize(buf);
            self.pts.serialize(buf);
        }
    }
    impl crate::Deserializable for ChannelState {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            let channel_id = i64::deserialize(buf)?;
            let pts = i32::deserialize(buf)?;
            Ok(ChannelState {
                channel_id,
                pts,
            })
        }
    }
    impl From<crate::enums::ChannelState> for ChannelState {
        fn from(x: crate::enums::ChannelState) -> Self {
            match x {
                crate::enums::ChannelState::State(x) => x,
            }
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct DataCenter {
        pub id: i32,
        pub ipv4: Option<i32>,
        pub ipv6: Option<[u8; 16]>,
        pub port: i32,
        pub auth: Option<Vec<u8>>,
    }
    impl crate::Identifiable for DataCenter {
        const CONSTRUCTOR_ID: u32 = 1970083510;
    }
    impl crate::Serializable for DataCenter {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            (0u32 | if self.ipv4.is_some() { 1 } else { 0 } | if self.ipv6.is_some() { 2 } else { 0 } | if self.auth.is_some() { 4 } else { 0 }).serialize(buf);
            self.id.serialize(buf);
            if let Some(ref x) = self.ipv4 { 
                x.serialize(buf);
            }
            if let Some(ref x) = self.ipv6 { 
                x.serialize(buf);
            }
            self.port.serialize(buf);
            if let Some(ref x) = self.auth { 
                x.serialize(buf);
            }
        }
    }
    impl crate::Deserializable for DataCenter {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            let flags = u32::deserialize(buf)?;
            let id = i32::deserialize(buf)?;
            let ipv4 = if (flags & 1) != 0 {
                Some(i32::deserialize(buf)?)
            } else {
                None
            };
            let ipv6 = if (flags & 2) != 0 {
                Some(<[u8; 16]>::deserialize(buf)?)
            } else {
                None
            };
            let port = i32::deserialize(buf)?;
            let auth = if (flags & 4) != 0 {
                Some(Vec::<u8>::deserialize(buf)?)
            } else {
                None
            };
            Ok(DataCenter {
                                id,
                ipv4,
                ipv6,
                port,
                auth,
            })
        }
    }
    impl From<crate::enums::DataCenter> for DataCenter {
        fn from(x: crate::enums::DataCenter) -> Self {
            match x {
                crate::enums::DataCenter::Center(x) => x,
            }
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct Session {
        pub dcs: Vec<crate::enums::DataCenter>,
        pub user: Option<crate::enums::User>,
        pub state: Option<crate::enums::UpdateState>,
    }
    impl crate::Identifiable for Session {
        const CONSTRUCTOR_ID: u32 = 2805905614;
    }
    impl crate::Serializable for Session {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            (0u32 | if self.user.is_some() { 1 } else { 0 } | if self.state.is_some() { 2 } else { 0 }).serialize(buf);
            self.dcs.serialize(buf);
            if let Some(ref x) = self.user { 
                x.serialize(buf);
            }
            if let Some(ref x) = self.state { 
                x.serialize(buf);
            }
        }
    }
    impl crate::Deserializable for Session {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            let flags = u32::deserialize(buf)?;
            let dcs = Vec::<crate::enums::DataCenter>::deserialize(buf)?;
            let user = if (flags & 1) != 0 {
                Some(crate::enums::User::deserialize(buf)?)
            } else {
                None
            };
            let state = if (flags & 2) != 0 {
                Some(crate::enums::UpdateState::deserialize(buf)?)
            } else {
                None
            };
            Ok(Session {
                                dcs,
                user,
                state,
            })
        }
    }
    impl From<crate::enums::Session> for Session {
        fn from(x: crate::enums::Session) -> Self {
            match x {
                crate::enums::Session::Session(x) => x,
            }
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct UpdateState {
        pub pts: i32,
        pub qts: i32,
        pub date: i32,
        pub seq: i32,
        pub channels: Vec<crate::enums::ChannelState>,
    }
    impl crate::Identifiable for UpdateState {
        const CONSTRUCTOR_ID: u32 = 3502955713;
    }
    impl crate::Serializable for UpdateState {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            self.pts.serialize(buf);
            self.qts.serialize(buf);
            self.date.serialize(buf);
            self.seq.serialize(buf);
            self.channels.serialize(buf);
        }
    }
    impl crate::Deserializable for UpdateState {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            let pts = i32::deserialize(buf)?;
            let qts = i32::deserialize(buf)?;
            let date = i32::deserialize(buf)?;
            let seq = i32::deserialize(buf)?;
            let channels = Vec::<crate::enums::ChannelState>::deserialize(buf)?;
            Ok(UpdateState {
                pts,
                qts,
                date,
                seq,
                channels,
            })
        }
    }
    impl From<crate::enums::UpdateState> for UpdateState {
        fn from(x: crate::enums::UpdateState) -> Self {
            match x {
                crate::enums::UpdateState::State(x) => x,
            }
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct User {
        pub id: i64,
        pub dc: i32,
        pub bot: bool,
    }
    impl crate::Identifiable for User {
        const CONSTRUCTOR_ID: u32 = 1731150477;
    }
    impl crate::Serializable for User {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            self.id.serialize(buf);
            self.dc.serialize(buf);
            self.bot.serialize(buf);
        }
    }
    impl crate::Deserializable for User {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            let id = i64::deserialize(buf)?;
            let dc = i32::deserialize(buf)?;
            let bot = bool::deserialize(buf)?;
            Ok(User {
                id,
                dc,
                bot,
            })
        }
    }
    impl From<crate::enums::User> for User {
        fn from(x: crate::enums::User) -> Self {
            match x {
                crate::enums::User::User(x) => x,
            }
        }
    }
}
/// This module contains all of the functions, each
/// represented by a `struct`. All of them implement
/// [`Identifiable`] and [`Serializable`].
///
/// To find out the type that Telegram will return upon
/// invoking one of these requests, check out the associated
/// type in the corresponding [`RemoteCall`] trait impl.
///
/// [`Identifiable`]: ../trait.Identifiable.html
/// [`Serializable`]: ../trait.Serializable.html
/// [`RemoteCall`]: trait.RemoteCall.html
#[allow(clippy::cognitive_complexity, clippy::identity_op, clippy::unreadable_literal)]
pub mod functions {
            
}
/// This module contains all of the boxed types, each
/// represented by a `enum`. All of them implement
/// [`Serializable`] and [`Deserializable`].
///
/// [`Serializable`]: /grammers_tl_types/trait.Serializable.html
/// [`Deserializable`]: /grammers_tl_types/trait.Deserializable.html
#[allow(clippy::large_enum_variant)]
pub mod enums {
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub enum ChannelState {
        State(crate::types::ChannelState),
    }
    impl crate::Serializable for ChannelState {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            use crate::Identifiable;
            match self {
                Self::State(x) => {
                    crate::types::ChannelState::CONSTRUCTOR_ID.serialize(buf);
                    x.serialize(buf)
                },
            }
        }
    }
    impl crate::Deserializable for ChannelState {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            use crate::Identifiable;
            let id = u32::deserialize(buf)?;
            Ok(match id {
                crate::types::ChannelState::CONSTRUCTOR_ID => Self::State(crate::types::ChannelState::deserialize(buf)?),
                _ => return Err(crate::deserialize::Error::UnexpectedConstructor { id }),
            })
        }
    }
    impl From<crate::types::ChannelState> for ChannelState {
        fn from(x: crate::types::ChannelState) -> Self {
            ChannelState::State(x)
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub enum DataCenter {
        Center(crate::types::DataCenter),
    }
    impl crate::Serializable for DataCenter {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            use crate::Identifiable;
            match self {
                Self::Center(x) => {
                    crate::types::DataCenter::CONSTRUCTOR_ID.serialize(buf);
                    x.serialize(buf)
                },
            }
        }
    }
    impl crate::Deserializable for DataCenter {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            use crate::Identifiable;
            let id = u32::deserialize(buf)?;
            Ok(match id {
                crate::types::DataCenter::CONSTRUCTOR_ID => Self::Center(crate::types::DataCenter::deserialize(buf)?),
                _ => return Err(crate::deserialize::Error::UnexpectedConstructor { id }),
            })
        }
    }
    impl From<crate::types::DataCenter> for DataCenter {
        fn from(x: crate::types::DataCenter) -> Self {
            DataCenter::Center(x)
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub enum Session {
        Session(crate::types::Session),
    }
    impl crate::Serializable for Session {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            use crate::Identifiable;
            match self {
                Self::Session(x) => {
                    crate::types::Session::CONSTRUCTOR_ID.serialize(buf);
                    x.serialize(buf)
                },
            }
        }
    }
    impl crate::Deserializable for Session {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            use crate::Identifiable;
            let id = u32::deserialize(buf)?;
            Ok(match id {
                crate::types::Session::CONSTRUCTOR_ID => Self::Session(crate::types::Session::deserialize(buf)?),
                _ => return Err(crate::deserialize::Error::UnexpectedConstructor { id }),
            })
        }
    }
    impl From<crate::types::Session> for Session {
        fn from(x: crate::types::Session) -> Self {
            Session::Session(x)
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub enum UpdateState {
        State(crate::types::UpdateState),
    }
    impl crate::Serializable for UpdateState {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            use crate::Identifiable;
            match self {
                Self::State(x) => {
                    crate::types::UpdateState::CONSTRUCTOR_ID.serialize(buf);
                    x.serialize(buf)
                },
            }
        }
    }
    impl crate::Deserializable for UpdateState {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            use crate::Identifiable;
            let id = u32::deserialize(buf)?;
            Ok(match id {
                crate::types::UpdateState::CONSTRUCTOR_ID => Self::State(crate::types::UpdateState::deserialize(buf)?),
                _ => return Err(crate::deserialize::Error::UnexpectedConstructor { id }),
            })
        }
    }
    impl From<crate::types::UpdateState> for UpdateState {
        fn from(x: crate::types::UpdateState) -> Self {
            UpdateState::State(x)
        }
    }
    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub enum User {
        User(crate::types::User),
    }
    impl crate::Serializable for User {
        fn serialize(&self, buf: &mut impl Extend<u8>) {
            use crate::Identifiable;
            match self {
                Self::User(x) => {
                    crate::types::User::CONSTRUCTOR_ID.serialize(buf);
                    x.serialize(buf)
                },
            }
        }
    }
    impl crate::Deserializable for User {
        fn deserialize(buf: crate::deserialize::Buffer) -> crate::deserialize::Result<Self> {
            use crate::Identifiable;
            let id = u32::deserialize(buf)?;
            Ok(match id {
                crate::types::User::CONSTRUCTOR_ID => Self::User(crate::types::User::deserialize(buf)?),
                _ => return Err(crate::deserialize::Error::UnexpectedConstructor { id }),
            })
        }
    }
    impl From<crate::types::User> for User {
        fn from(x: crate::types::User) -> Self {
            User::User(x)
        }
    }
}
