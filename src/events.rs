//
// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use api::{Message, File, Channel, User};
//use api::mods::{Item, Comment};
use api::types::Bot;
use rustc_serialize::{Decodable, Decoder};

/// Represents Slack [rtm event](https://api.slack.com/rtm) types.
#[derive(Clone,Debug)]
pub enum Event {
    /// Represents the slack [`hello`](https://api.slack.com/events/hello) event.
    Hello,
    /// Represents the slack [`message`](https://api.slack.com/events/message)
    /// event.
    Message(Message),
    /// Represents the slack
    /// [`user_typing`](https://api.slack.com/events/user_typing) event.
    UserTyping {
        channel: String,
        user: String,
    },
    /// Represents the slack
    /// [`channel_marked`](https://api.slack.com/events/channel_marked) event.
    ChannelMarked {
        channel: String,
        ts: String,
    },
    /// Represents the slack
    /// [`channel_created`](https://api.slack.com/events/channel_created) event.
    ChannelCreated {
        channel: Channel,
    },
    /// Represents the slack
    /// [`channel_joined`](https://api.slack.com/events/channel_joined) event.
    ChannelJoined {
        channel: Channel,
    },
    /// Represents the slack
    /// [`channel_left`](https://api.slack.com/events/channel_left) event.
    ChannelLeft {
        channel: String,
    },
    /// Represents the slack
    /// [`channel_deleted`](https://api.slack.com/events/channel_deleted) event.
    ChannelDeleted {
        channel: String,
    },
    /// Represents the slack
    /// [`channel_rename`](https://api.slack.com/events/channel_rename) event.
    ChannelRename {
        channel: Channel,
    },
    /// Represents the slack
    /// [`channel_archive`](https://api.slack.com/events/channel_archive) event.
    ChannelArchive {
        channel: String,
        user: String,
    },
    /// Represents the slack
    /// [`channel_unarchive`](https://api.slack.com/events/channel_unarchive) event.
    ChannelUnArchive {
        channel: String,
        user: String,
    },
    /// Represents the slack
    /// [`channel_history_changed`](https://api.slack.
    /// com/events/channel_history_changed) event.
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`im_created`](https://api.slack.com/events/im_created) event.
    ImCreated {
        user: String,
        channel: Channel,
    },
    /// Represents the slack [`im_open`](https://api.slack.com/events/im_open)
    /// event.
    ImOpen {
        user: String,
        channel: String,
    },
    /// Represents the slack [`im_close`](https://api.slack.com/events/im_close)
    /// event.
    ImClose {
        user: String,
        channel: String,
    },
    /// Represents the slack [`im_marked`](https://api.slack.com/events/im_marked)
    /// event.
    ImMarked {
        channel: String,
        ts: String,
    },
    /// Represents the slack
    /// [`im_history_changed`](https://api.slack.com/events/im_history_changed)
    /// event.
    ImHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`group_joined`](https://api.slack.com/events/group_joined) event.
    GroupJoined {
        channel: Channel,
    },
    /// Represents the slack
    /// [`group_left`](https://api.slack.com/events/group_left) event.
    GroupLeft {
        channel: Channel,
    },
    /// Represents the slack
    /// [`group_open`](https://api.slack.com/events/group_open) event.
    GroupOpen {
        user: String,
        channel: String,
    },
    /// Represents the slack
    /// [`group_close`](https://api.slack.com/events/group_close) event.
    GroupClose {
        user: String,
        channel: String,
    },
    /// Represents the slack
    /// [`group_archive`](https://api.slack.com/events/group_archive) event.
    GroupArchive {
        channel: String,
    },
    /// Represents the slack
    /// [`group_unarchive`](https://api.slack.com/events/group_unarchive) event.
    GroupUnArchive {
        channel: String,
    },
    /// Represents the slack
    /// [`group_rename`](https://api.slack.com/events/group_rename) event.
    GroupRename {
        channel: Channel,
    },
    /// Represents the slack
    /// [`group_marked`](https://api.slack.com/events/group_marked) event.
    GroupMarked {
        channel: String,
        ts: String,
    },
    /// Represents the slack
    /// [`group_history_changed`](https://api.slack.
    /// com/events/group_history_changed) event.
    GroupHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`file_created`](https://api.slack.com/events/file_created) event.
    FileCreated {
        file: File,
    },
    /// Represents the slack
    /// [`file_shared`](https://api.slack.com/events/file_shared) event.
    FileShared {
        file: File,
    },
    /// Represents the slack
    /// [`file_unshared`](https://api.slack.com/events/file_unshared) event.
    FileUnShared {
        file: File,
    },
    /// Represents the slack
    /// [`file_public`](https://api.slack.com/events/file_public) event.
    FilePublic {
        file: File,
    },
    /// Represents the slack
    /// [`file_private`](https://api.slack.com/events/file_private) event.
    FilePrivate {
        file: String,
    },
    /// Represents the slack
    /// [`file_change`](https://api.slack.com/events/file_change) event.
    FileChange {
        file: File,
    },
    /// Represents the slack
    /// [`file_deleted`](https://api.slack.com/events/file_deleted) event.
    FileDeleted {
        file_id: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`file_comment_added`](https://api.slack.com/events/file_comment_added)
    /// event.
    FileCommentAdded {
        file: File,
        comment: Comment,
    },
    /// Represents the slack
    /// [`file_comment_edited`](https://api.slack.com/events/file_comment_edited)
    /// event.
    FileCommentEdited {
        file: File,
        comment: Comment,
    },
    /// Represents the slack
    /// [`file_comment_deleted`](https://api.slack.com/events/file_comment_deleted)
    /// event.
    FileCommentDeleted {
        file: File,
        comment: String,
    },
    /// Represents the slack [`pin_added`](https://api.slack.com/events/pin_added)
    /// event.
    PinAdded {
        user: String,
        channel_id: String,
        item: Item,
        event_ts: String,
    },
    /// Represents the slack
    /// [`pin_removed`](https://api.slack.com/events/pin_removed) event.
    PinRemoved {
        user: String,
        channel_id: String,
        item: Item,
        has_pins: bool,
        event_ts: String,
    },
    /// Represents the slack
    /// [`presence_change`](https://api.slack.com/events/presence_change) event.
    PresenceChange {
        user: String,
        presence: String,
    },
    /// Represents the slack
    /// [`manual_presence_change`](https://api.slack.
    /// com/events/manual_presence_change) event.
    ManualPresenceChange {
        presence: String,
    },
    /// Represents the slack
    /// [`pref_change`](https://api.slack.com/events/pref_change) event.
    PrefChange {
        name: String,
        value: String,
    },
    /// Represents the slack
    /// [`user_change`](https://api.slack.com/events/user_change) event.
    UserChange {
        user: User,
    },
    /// Represents the slack [`team_join`](https://api.slack.com/events/team_join)
    /// event.
    TeamJoin {
        user: User,
    },
    /// Represents the slack
    /// [`star_added`](https://api.slack.com/events/star_added) event.
    StarAdded {
        user: String,
        item: Item,
        event_ts: String,
    },
    /// Represents the slack
    /// [`star_removed`](https://api.slack.com/events/star_removed) event.
    StarRemoved {
        user: String,
        item: Item,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_added`](https://api.slack.com/events/reaction_added) event.
    ReactionAdded {
        user: String,
        reaction: String,
        item: Item,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_removed`](https://api.slack.com/events/reaction_removed) event.
    ReactionRemoved {
        user: String,
        reaction: String,
        item: Item,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`emoji_changed`](https://api.slack.com/event/emoji_changed) event.
    EmojiChanged {
        event_ts: String,
    },
    /// Represents the slack
    /// [`commands_changed`](https://api.slack.com/event/commands_changed) event.
    CommandsChanged {
        event_ts: String,
    },
    /// Represents the slack
    /// [`team_plan_change`](https://api.slack.com/event/team_plan_change) event.
    TeamPlanChange {
        plan: String,
    },
    /// Represents the slack
    /// [`team_pref_change`](https://api.slack.com/event/team_pref_change) event.
    TeamPrefChange {
        name: String,
        value: bool,
    },
    /// Represents the slack
    /// [`team_rename`](https://api.slack.com/event/team_rename) event.
    TeamRename {
        name: String,
    },
    /// Represents the slack
    /// [`team_domain_change`](https://api.slack.com/event/team_domain_change)
    /// event.
    TeamDomainChange {
        url: String,
        domain: String,
    },
    /// Represents the slack
    /// [`email_domain_changeed`](https://api.slack.
    /// com/event/email_domain_changeed) event.
    EmailDomainChanged {
        email_domain: String,
        event_ts: String,
    },
    /// Represents the slack [`bot_added`](https://api.slack.com/event/bot_added)
    /// event.
    BotAdded {
        bot: Bot,
    },
    /// Represents the slack
    /// [`bot_changed`](https://api.slack.com/event/bot_changed) event.
    BotChanged {
        bot: Bot,
    },
    /// Represents the slack
    /// [`accounts_changed`](https://api.slack.com/event/accounts_changed) event.
    AccountsChanged,
    /// Represents the slack
    /// [`team_migration_started`](https://api.slack.
    /// com/event/team_migration_started) event.
    TeamMigrationStarted,
    /// Represents the slack
    /// [`reconnect_url`](https://api.slack.com/event/reconnect_url)
    /// event.
    ReconnectUrl,
    /// Represents a confirmation of a message sent
    MessageSent {
        reply_to: isize,
        ts: String,
        text: String,
    },
    /// Represents an error sending a message
    MessageError {
        reply_to: isize,
        code: isize,
        message: String,
    },
}

impl Decodable for Event {
    fn decode<D: Decoder>(d: &mut D) -> Result<Event, D::Error> {
        let ty: Option<String> = try!(d.read_struct_field("type", 0, |d| Decodable::decode(d)));
        match ty {
            Some(ty) => {
                match ty.as_ref() {
                    "hello" => Ok(Event::Hello),
                    "message" => Ok(Event::Message(try!(Message::decode(d)))),
                    "user_typing" => Ok(Event::UserTyping {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_marked" => Ok(Event::ChannelMarked {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_created" => Ok(Event::ChannelCreated {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_joined" => Ok(Event::ChannelJoined {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_left" => Ok(Event::ChannelLeft {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_deleted" => Ok(Event::ChannelDeleted {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_rename" => Ok(Event::ChannelRename {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_archive" => Ok(Event::ChannelArchive {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_unarchive" => Ok(Event::ChannelUnArchive {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    }),
                    "channel_history_changed" => Ok(Event::ChannelHistoryChanged {
                        latest: try!(d.read_struct_field("latest", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "im_created" => Ok(Event::ImCreated {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "im_open" => Ok(Event::ImOpen {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "im_close" => Ok(Event::ImClose {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "im_marked" => Ok(Event::ImMarked {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    }),
                    "im_history_changed" => Ok(Event::ImHistoryChanged {
                        latest: try!(d.read_struct_field("latest", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "group_joined" => Ok(Event::GroupJoined {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_left" => Ok(Event::GroupLeft {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_open" => Ok(Event::GroupOpen {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_close" => Ok(Event::GroupClose {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_archive" => Ok(Event::GroupArchive {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_unarchive" => Ok(Event::GroupUnArchive {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_rename" => Ok(Event::GroupRename {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "group_marked" => Ok(Event::GroupMarked {
                        channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    }),
                    "group_history_changed" => Ok(Event::GroupHistoryChanged {
                        latest: try!(d.read_struct_field("latest", 0, |d| Decodable::decode(d))),
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    }),
                    "file_created" => Ok(Event::FileCreated {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_shared" => Ok(Event::FileShared {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_unshared" => Ok(Event::FileUnShared {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_public" => Ok(Event::FilePublic {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_private" => Ok(Event::FilePrivate {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_change" => Ok(Event::FileChange {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    }),
                    "file_deleted" => Ok(Event::FileDeleted {
                        file_id: try!(d.read_struct_field("file_id", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "file_comment_added" => Ok(Event::FileCommentAdded {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                        comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d))),
                    }),
                    "file_comment_edited" => Ok(Event::FileCommentEdited {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                        comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d))),
                    }),
                    "file_comment_deleted" => Ok(Event::FileCommentDeleted {
                        file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                        comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d))),
                    }),
                    "pin_added" => Ok(Event::PinAdded {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel_id: try!(d.read_struct_field("channel_id", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "pin_removed" => Ok(Event::PinRemoved {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        channel_id: try!(d.read_struct_field("channel_id", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        has_pins: try!(d.read_struct_field("has_pins", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "presence_change" => Ok(Event::PresenceChange {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        presence: try!(d.read_struct_field("presence", 0, |d| Decodable::decode(d))),
                    }),
                    "manual_presence_change" => Ok(Event::ManualPresenceChange {
                        presence: try!(d.read_struct_field("presence", 0, |d| Decodable::decode(d))),
                    }),
                    "pref_change" => Ok(Event::PrefChange {
                        name: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                        value: try!(d.read_struct_field("value", 0, |d| Decodable::decode(d))),
                    }),
                    "user_change" => Ok(Event::UserChange {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    }),
                    "team_join" => Ok(Event::TeamJoin {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    }),
                    "star_added" => Ok(Event::StarAdded {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "star_removed" => Ok(Event::StarRemoved {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "reaction_added" => Ok(Event::ReactionAdded {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        reaction: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        item_user: try!(d.read_struct_field("item_user", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "reaction_removed" => Ok(Event::ReactionRemoved {
                        user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                        reaction: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                        item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                        item_user: try!(d.read_struct_field("item_user", 0, |d| Decodable::decode(d))),
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                      }),
                    "emoji_changed" => Ok(Event::EmojiChanged {
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "commands_changed" => Ok(Event::CommandsChanged {
                        event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                    }),
                    "team_plan_change" => Ok(Event::TeamPlanChange {
                        plan: try!(d.read_struct_field("plan", 0, |d| Decodable::decode(d))),
                    }),
                    "team_pref_change" => Ok(Event::TeamPrefChange {
                        name: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                        value: try!(d.read_struct_field("value", 0, |d| Decodable::decode(d))),
                    }),
                    "team_rename" => Ok(Event::TeamRename {
                        name: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                    }),
                    "team_domain_change" => Ok(Event::TeamDomainChange {
                        url: try!(d.read_struct_field("url", 0, |d| Decodable::decode(d))),
                        domain: try!(d.read_struct_field("domain", 0, |d| Decodable::decode(d))),
                    }),
                    "email_domain_changeed" =>
                        Ok(Event::EmailDomainChanged {
                            email_domain: try!(d.read_struct_field("email_domain",
                                                                   0,
                                                                   |d| Decodable::decode(d))),
                            event_ts: try!(d.read_struct_field("event_ts", 0, |d| Decodable::decode(d))),
                        }),
                    "bot_added" => Ok(Event::BotAdded {
                        bot: try!(d.read_struct_field("bot", 0, |d| Decodable::decode(d))),
                    }),
                    "bot_changed" => Ok(Event::BotChanged {
                        bot: try!(d.read_struct_field("bot", 0, |d| Decodable::decode(d))),
                    }),
                    "accounts_changed" => Ok(Event::AccountsChanged),
                    "team_migration_started" => Ok(Event::TeamMigrationStarted),
                    "reconnect_url" => Ok(Event::ReconnectUrl),
                    _ => Err(d.error(&format!("Unknown Message type: {}", ty))),
                }
            }
            None => {
                // message confirmations don't have a type field
                let ok: bool = try!(d.read_struct_field("ok", 0, |d| Decodable::decode(d)));
                let reply_to: isize = try!(d.read_struct_field("reply_to", 0, |d| Decodable::decode(d)));
                if ok {
                    Ok(Event::MessageSent {
                        reply_to: reply_to,
                        ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                        text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    })
                } else {
                    d.read_struct_field("error", 0, |d| {
                        Ok(Event::MessageError {
                            reply_to: reply_to,
                            code: try!(d.read_struct_field("code", 0, |d| Decodable::decode(d))),
                            message: try!(d.read_struct_field("msg", 0, |d| Decodable::decode(d))),
                        })
                    })
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use api::Message;
    use rustc_serialize::json;

    #[test]
    fn decode_short_standard_message() {
        let event: Event = json::decode(r#"{
            "type": "message",
            "ts": "1234567890.218332",
            "user": "U12345678",
            "text": "Hello world",
            "channel": "C12345678"
        }"#)
                               .unwrap();
        match event {
            Event::Message(message) => {
                match message {
                    Message::Standard { ts, channel: _, user, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                        assert_eq!(ts, "1234567890.218332");
                        assert_eq!(text.unwrap(), "Hello world");
                        assert_eq!(user.unwrap(), "U12345678");
                    }
                    _ => panic!("Message decoded into incorrect variant."),
                }
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_sent_ok() {
        let event: Event = json::decode(r#"{
            "ok": true,
            "reply_to": 1,
            "ts": "1234567890.218332",
            "text": "Hello world"
        }"#).unwrap();
        match event {
            Event::MessageSent{reply_to, ts, text} => {
                assert_eq!(reply_to, 1);
                assert_eq!(ts, "1234567890.218332");
                assert_eq!(text, "Hello world");
            },
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_sent_not_ok() {
        let event: Event = json::decode(r#"{
            "ok": false,
            "reply_to": 1,
            "error": {
                "code": 2,
                "msg": "message text is missing"
            }
        }"#).unwrap();
        match event {
            Event::MessageError{reply_to, code, message} => {
                assert_eq!(reply_to, 1);
                assert_eq!(code, 2);
                assert_eq!(message, "message text is missing");
            },
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_extended_standard_message() {
        let event: Event = json::decode(r##"{
            "type": "message",
            "ts": "1234567890.218332",
            "user": "U12345678",
            "text": "Hello world",
            "channel": "C12345678",
            "is_starred": false,
            "pinned_to": [ "C12345678" ],
            "reactions": [
                {
                    "name": "astonished",
                    "count": 5,
                    "users": [ "U12345678", "U87654321" ]
                }
            ],
            "edited": {
                "user": "U12345678",
                "ts": "1234567890.218332"
            },
            "attachments": [
                {
                    "fallback": "Required plain-text summary of the attachment.",
                    "color": "#36a64f",
                    "pretext": "Optional text that appears above the attachment block",
                    "author_name": "Bobby Tables",
                    "author_link": "http://flickr.com/bobby/",
                    "author_icon": "http://flickr.com/icons/bobby.jpg",
                    "title": "Slack API Documentation",
                    "title_link": "https://api.slack.com/",
                    "text": "Optional text that appears within the attachment",
                    "fields": [
                        {
                            "title": "Priority",
                            "value": "High",
                            "short": false
                        }
                    ],
                    "image_url": "http://my-website.com/path/to/image.jpg",
                    "thumb_url": "http://example.com/path/to/thumb.png"
                }
            ]
        }"##)
                               .unwrap();
        match event {
            Event::Message(message) => {
                match message {
                    Message::Standard { ts: _, channel: _, user: _, text: _, is_starred, pinned_to: _, reactions: _, edited: _, attachments } => {
                        assert_eq!(is_starred, Some(false));
                        assert_eq!(attachments.unwrap()[0].color.as_ref().unwrap(), "#36a64f");
                    }
                    _ => panic!("Message decoded into incorrect variant."),
                }
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

}
